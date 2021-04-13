use clap::{App, AppSettings, Arg};
use regex::Regex;
use rusqlite::{named_params, Connection, Result, NO_PARAMS};
use std::error;

// Regex pattern for query string
const PATTERN: &str = r"^(?P<book>\w+)( (?P<start_chapter>\d+)(:(?P<start_verse>\d+)(-((?P<end_chapter>\d+):)?(?P<end_verse>\d+))?)?)?$";

// Struct for capturing query parameters
struct Range<'t> {
    book: &'t str,
    start_chapter: i32,
    end_chapter: i32,
    start_verse: i32,
    end_verse: i32,
}

fn main() -> Result<(), Box<dyn error::Error>> {

    // Generate and capture usage
    let matches = App::new("dra-cli")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::TrailingVarArg)
        .version("0.1.0")
        .about("Command-line interface for Douay-Rheims American Bible")
        .arg(
            Arg::new("books")
                .short('b')
                .long("books")
                .takes_value(false)
                .about("Lists the available books"),
        )
        .arg(
            Arg::new("QUERY")
                .required(true)
                .conflicts_with("books")
                .takes_value(true)
                .multiple(true)
                .about("Query string:\n\t<book code> <chapter>:<verse>\n\t<book code> <chapter>:<start_verse>-<end_verse>\n\t<book code> <chapter>:<start_verse>-<end_chapter>:<end_verse>"),
        )
        .get_matches();

    // If -b then list books
    if matches.is_present("books") {
        return list_books();
    }
    // If query present then try to print verses
    if matches.is_present("QUERY") {
        let query_list: Vec<&str> = matches
            .values_of("QUERY")
            .ok_or("No query string")?
            .collect();
        let query = &query_list.join(" ");
        // Capture query parameters or return error if invalid
        let range = parse_query(query)?;
        // Print verses based on range
        return print_verses(&range);
    }

    Ok(())
}

// Lists the books in books table
fn list_books() -> Result<(), Box<dyn error::Error>> {
    let conn = Connection::open("dra.db")?;

    println!("The following books are available:");

    let mut stmt = conn.prepare("SELECT code, long FROM books")?;
    let mut rows = stmt.query(NO_PARAMS)?;
    while let Some(row) = rows.next()? {
        println!(
            "\t{}\t{}",
            row.get::<usize, Box<str>>(0)?,
            row.get::<usize, Box<str>>(1)?
        );
    }

    Ok(())
}

// Parses query using regex
// Returns captured parameters if query valid, error otherwise
fn parse_query(query: &str) -> Result<Range, Box<dyn error::Error>> {
    let pattern = Regex::new(PATTERN)?;
    let capture = pattern.captures(query).ok_or("Invalid query string")?;

    let book = capture
        .name("book")
        .ok_or("Query string does not contain book")?
        .as_str();
    let start_chapter = capture
        .name("start_chapter")
        .ok_or("Query string does not contain chapter")?
        .as_str()
        .trim()
        .parse()?;
    let end_chapter = match capture.name("end_chapter") {
        Some(ch2) => ch2.as_str().trim().parse()?,
        None => start_chapter,
    };
    let start_verse_present: bool;
    let start_verse = match capture.name("start_verse") {
        Some(v1) => {
            start_verse_present = true;
            v1.as_str().trim().parse()?
        },
        None => {
            start_verse_present = false;
            1
        },
    };
    let end_verse = match capture.name("end_verse") {
        Some(v2) => v2.as_str().trim().parse()?,
        None if start_verse_present => start_verse,
        _ => 200
    };
    Ok(Range {
        book,
        start_chapter,
        end_chapter,
        start_verse,
        end_verse,
    })
}

// Prints verses based on range
fn print_verses(range: &Range) -> Result<(), Box<dyn error::Error>> {
    if range.end_chapter < range.start_chapter {
        Err("Query range invalid: end_chapter precedes start_chapter")?
    } else if range.end_chapter == range.start_chapter && range.end_verse < range.start_verse {
        Err("Query range invalid: end_verse precedes start_verse")?
    }

    let conn = Connection::open("dra.db")?;

    let mut stmt = conn.prepare("SELECT chapter, startVerse, verseText FROM engDRA_vpl WHERE rowid BETWEEN (SELECT MIN(rowid) FROM engDRA_vpl WHERE book = :book AND chapter >= :start_chapter AND startVerse >= :start_verse) AND (SELECT MAX(rowid) FROM engDRA_vpl WHERE book = :book AND chapter <= :end_chapter AND startVerse <= :end_verse)")?;
    let mut rows = stmt.query_named(named_params!{":book":range.book, ":start_chapter":range.start_chapter, ":end_chapter":range.end_chapter, ":start_verse":range.start_verse, ":end_verse":range.end_verse})?;

    while let Some(row) = rows.next()? {
        println!(
            "{:7} {}",
            format!(
                "{}:{}",
                row.get::<usize, i32>(0)?,
                row.get::<usize, i32>(1)?
            ),
            row.get::<usize, Box<str>>(2)?
        );
    }

    Ok(())
}
