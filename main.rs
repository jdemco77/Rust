use std::env;   //env return iterator of command line args passed
use std::fs;    //file system import
use std::process;
use std::error::Error;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // collect() turns an iterator into a colllection

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    }); 

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

    run(config);

    //takes path,opens file, returns std::io::Result<String>
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");     
}




