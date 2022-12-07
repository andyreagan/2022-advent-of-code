use std::fs;

use day01::search;


fn main() {
    let file_path = "input1";

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).unwrap();
    // println!("With text:\n{contents}");

    let result = search(&contents);

    println!("Most calories:\n{result}");
}