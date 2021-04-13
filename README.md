# dra-cli  
Steps to run:  
cargo build --release  
./target/release/dra-cli.exe [FLAGS] &lt;QUERY>  

USAGE:  
&nbsp;dra-cli.exe [FLAGS] &lt;QUERY>...

ARGS:  
&emsp;&lt;QUERY>...&emsp;Query string:  
&emsp;&emsp;&lt;book code> &lt;chapter>:&lt;verse>  
&emsp;&emsp;&lt;book code> &lt;chapter>:&lt;start_verse>-&lt;end_verse>  
&emsp;&emsp;&lt;book code> &lt;chapter>:&lt;start_verse>-&lt;end_chapter>:&lt;end_verse>  

FLAGS:  
&emsp;-b, --books      Lists the available books  
&emsp;-h, --help       Prints help information  
&emsp;-V, --version    Prints version information  
