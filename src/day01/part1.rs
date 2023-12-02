// https://adventofcode.com/2023/day/1

use crate::file_reader::file_reader;

pub fn solve() {
    let lines = file_reader::read_as_vec("inputs/day01.txt");

    let mut total = 0;

    for line in lines.iter() {
        let mut line_clone = line.clone();
        line_clone.retain(|c| c.is_numeric());

        let first_char = line_clone.chars().nth(0).unwrap();
        let last_char = line_clone.chars().nth(line_clone.len() - 1).unwrap();

        total += format!("{}{}", first_char, last_char)
            .parse::<i32>()
            .unwrap();
    }

    println!("Day 01 Part 1: {}", total);
}
