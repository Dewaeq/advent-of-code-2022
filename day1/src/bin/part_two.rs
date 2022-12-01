use std::fs;
use day1::part_two;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("{}", part_two(&input))
}
