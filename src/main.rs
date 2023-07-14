use std::fs;
use aoc_7;

fn main() {
    println!("Hello, world!");
    part_1();
}

fn part_1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = aoc_7::process_part_1(&input);
    println!("{}", result);
}