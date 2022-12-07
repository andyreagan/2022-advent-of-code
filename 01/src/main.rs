use std::fs;

use day01::search;
use day01::top3;


fn main() {
    let file_path = "input1";

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).unwrap();
    // println!("With text:\n{contents}");

    let result = search(&contents);

    println!("Most calories:\n{result}");

    let result = top3(&contents);

    println!("Top 3 most calories:\n{result}");
}