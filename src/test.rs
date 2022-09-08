
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