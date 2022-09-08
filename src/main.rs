use std::env;
use std::process;

use rust_testing::{Input, run};


fn main() {
    let args: Vec<String> = env::args().collect();

    let input: Input = Input::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(0);
    });

    println!("Search Keyword: {}", input.query);
    println!("Filename: {}", input.file_name);

    if let Err(e) = run(input) {
        eprintln!("Application error: {}", e);
        process::exit(0);
    }
}







