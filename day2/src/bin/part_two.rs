use std::fs;
use day2::*;

fn main() {
    let input = fs::read_to_string("./INPUT.txt").unwrap();
    println!("Method 1: {}", method_one::part_two(&input));
    println!("Method 2: {}", method_two::part_two(&input));
}