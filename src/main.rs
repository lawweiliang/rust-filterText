use std::env;
use std::error::Error;
use std::fs;
use std::process;


fn main() {

    // panic!("Problem");
    // process::exit(101);
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


fn run(input: Input) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(input.file_name)?;
    println!("With contents:\n {}", contents);

    Ok(())
}


struct Input {
    query: String,
    file_name: String,
}

impl Input {
    fn new(args: Vec<String>) -> Result<Input, String> {
        if args.len() < 3 {
            return Err(String::from("not enough arguments"));
        }

        let query: String = args[1].clone();
        let file_name: String = args[2].clone();

        Ok(Input { query, file_name })
    }
}





