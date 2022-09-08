use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query: &String = &args[1];
    let filename: &String = &args[2];

    println!("Searching for {}", query);
    println!("Filename {}", filename);

    let contents = fs::read_to_string(filename).expect("Something wrong with the file.");
    println!("With contents:\n {}", contents);

    
    
}
