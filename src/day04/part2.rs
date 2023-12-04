use std::collections::{HashMap, HashSet};

use crate::file_reader::file_reader;

fn get_current_nums_hash(current_numbers: &Vec<&str>) -> HashSet<String> {
    let mut current_nums_hash = HashSet::new();

    for num in current_numbers {
        current_nums_hash.insert(num.clone().to_string());
    }

    current_nums_hash
}

pub fn solve() {
    let lines = file_reader::read_as_vec("inputs/day04.txt");

    let mut card_copies: HashMap<usize, usize> = HashMap::new();
    card_copies.insert(0, 1); // Insert the first card

    let mut total = 0;

    for (i, line) in lines.iter().enumerate() {
        let mut game_split = line.split(":");
        let game_label = game_split.next().unwrap().trim();

        println!("Playing {}", game_label);

        let mut cards = game_split.next().unwrap().split("|");

        // First half is winning numbers, second half is current numbers
        let winning_numbers: Vec<&str> = cards.next().unwrap().split_whitespace().collect();
        let current_numbers: Vec<&str> = cards.next().unwrap().split_whitespace().collect();

        let current_nums_hash = get_current_nums_hash(&current_numbers);

        let num_plays = match card_copies.get(&i) {
            None => {
                card_copies.insert(i, 1);
                1
            }
            Some(num) => *num,
        };

        let end_orig_index = i + num_plays;

        // println!("{}: plays {} times", game_label, num_plays);

        // This is how many times we are playing the current card
        for _ in i..end_orig_index {
            // Now lets check for matches with the winning numbers
            let mut num_matches = 0;
            for num in winning_numbers.iter() {
                if current_nums_hash.contains(*num) {
                    // println!("{}: {}", game_label, num);
                    num_matches += 1;
                }
            }

            let last_index = if i + num_matches + 1 > lines.len() {
                lines.len()
            } else {
                i + num_matches + 1
            };

            // Now we need to iterate over the number of matches and add it to the key
            // in the hashmap
            for k in i + 1..last_index {
                // println!("{}: Adding to game {}", game_label, k + 1);
                let num_plays = match card_copies.get(&k) {
                    None => 1,
                    Some(num) => *num,
                };

                card_copies.insert(k, num_plays + 1);
            }
        }
    }

    for (k, v) in card_copies.iter() {
        /* if k >= &games_played {
            continue;
        } */
        println!("{}: {}", k, v);
        total += v;
    }

    // println!("{:?}", card_copies);
    println!("Day 04, Part 1: {}", total);
}
