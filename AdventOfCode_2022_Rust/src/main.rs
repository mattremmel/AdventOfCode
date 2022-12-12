use crate::common::Challenge;
use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use std::fs;

mod common;
mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    // Day 1
    let day1_input = fs::read_to_string("./inputs/day1.txt").unwrap();
    println!("Day 1 - Part 1: {}", Day1::part1(&day1_input));
    println!("Day 1 - Part 2: {}", Day1::part2(&day1_input));

    // Day 2
    let day2_input = fs::read_to_string("./inputs/day2.txt").unwrap();
    println!("Day 2 - Part 1: {}", Day2::part1(&day2_input));
    println!("Day 2 - Part 2: {}", Day2::part2(&day2_input));

    // Day 3
    let day3_input = fs::read_to_string("./inputs/day3.txt").unwrap();
    println!("Day 3 - Part 1: {}", Day3::part1(&day3_input));
    println!("Day 3 - Part 2: {}", Day3::part2(&day3_input));

    // Day 4
    let day4_input = fs::read_to_string("./inputs/day4.txt").unwrap();
    println!("Day 4 - Part 1: {}", Day4::part1(&day4_input));
    println!("Day 4 - Part 2: {}", Day4::part2(&day4_input));
}
