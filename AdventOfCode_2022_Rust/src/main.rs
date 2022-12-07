use crate::common::Challenge;
use day1::Day1;
use std::fs;

mod common;
mod day1;

fn main() {
    // Day 1
    let day1_input = fs::read_to_string("./inputs/day1.txt").unwrap();
    println!("Day 1 - Part 1: {}", Day1::part1(&day1_input));
    println!("Day 1 - Part 2: {}", Day1::part2(&day1_input));

}
