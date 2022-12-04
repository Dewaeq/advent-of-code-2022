use std::fs;
use day3::part_two;

fn main() {
    let input = fs::read_to_string("./INPUT.txt").unwrap();
    println!("{}", part_two(&input));
}