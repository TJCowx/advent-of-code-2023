use std::collections::HashSet;

use crate::file_reader::file_reader;

pub fn solve() {
    let lines = file_reader::read_as_vec("inputs/day04.txt");

    let mut total = 0;

    for line in lines.iter() {
        let mut game_split = line.split(":");
        let game_label = game_split.next().unwrap().trim();
        let mut cards = game_split.next().unwrap().split("|");

        // First half is winning numbers, second half is current numbers
        let winning_numbers: Vec<&str> = cards.next().unwrap().split_whitespace().collect();
        let current_numbers: Vec<&str> = cards.next().unwrap().split_whitespace().collect();

        let mut current_nums_hash = HashSet::new();

        for num in current_numbers {
            current_nums_hash.insert(num);
        }

        let mut num_matches = 0;
        for num in winning_numbers {
            if current_nums_hash.contains(num) {
                num_matches += 1;
            }
        }

        println!("{}: {}", game_label, num_matches);

        total += match num_matches {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 4,
            4 => 8,
            5 => 16,
            6 => 32,
            7 => 64,
            8 => 128,
            9 => 256,
            10 => 512,
            _ => 0,
        };
    }

    println!("Day 04, Part 1: {}", total);
}
