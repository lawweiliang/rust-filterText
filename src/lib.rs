use std::error::Error;
use std::fs;
use crate::error::error_print;


pub fn run(input: Input) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(input.file_name)?;

    println!("Search Result:\n ");
    for line in search(input.query, contents) {
        println!("{}", line);
    }

    return Ok(());
}

pub fn search(query: String, contents: String) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line.to_string());
        }
    }
    return results;
}

pub struct Input {
    pub query: String,
    pub file_name: String,
}

impl Input {
    pub fn new(args: Vec<String>) -> Result<Input, String> {
        if args.len() < 3 {
            return Err(String::from("not enough arguments"));
        }

        let query: String = args[1].clone();
        let file_name: String = args[2].clone();

        return Ok(Input { query, file_name });
    }
}


/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_test() {
        let query: String = String::from("we");
        let contents: String = String::from("
We are going to the moon.
Hello World.
Name is AliWe.");

        assert_eq!(vec!["We are going to the moon.", "Name is AliWe."], search(query, contents));
    }
}*/

//Add Testing Module
#[cfg(test)]
mod test;