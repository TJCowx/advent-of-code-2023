use std::{borrow::BorrowMut, collections::HashMap};

use crate::file_reader::file_reader;

fn get_col(lines: &Vec<String>, col: usize) -> String {
    String::from_iter(lines.iter().map(|line| line.chars().nth(col).unwrap()))
}

fn tilt_col(col: String) -> usize {
    let mut col_sum = 0;

    let col_len = col.len();

    // Split up by '#'
    let splits = col.split('#').into_iter().collect::<Vec<&str>>();
    let mut prev_len = 0;

    for split in splits.iter() {
        // Split is empty so we can skip
        if split.is_empty() {
            prev_len += 1;
            continue;
        }

        match move_split(split, prev_len, col_len) {
            Some(move_load) => {
                // Add to the sum
                col_sum += move_load;
            }
            None => {}
        }

        prev_len += split.len() + 1;
    }

    col_sum
}

fn move_split(split: &str, prev_len: usize, col_len: usize) -> Option<usize> {
    if split.is_empty() {
        return None;
    }

    let mut chars: Vec<char> = split.chars().collect();
    let mut load = 0;

    for i in 0..chars.len() {
        if chars[i] == 'O' {
            // Calculate the modifier
            load += col_len - prev_len - i;
            continue;
        }

        // Find the next 0
        let next_stone = chars.iter().skip(i + 1).position(|c| *c == 'O');

        // We hit the last 0 in the split lets move on
        if next_stone.is_none() {
            break;
        }

        let next_stone = next_stone.unwrap() + i + 1;

        load += col_len - prev_len - i;
        // Now we move the 0 to the current index
        chars[i] = 'O';
        chars[next_stone] = '.';
    }

    Some(load)
}

pub fn solve() {
    let lines = file_reader::read_as_vec("inputs/day14.txt");

    let mut sum = 0;

    // TODO: Loop through each column and tilt
    for i in 0..lines[0].len() {
        let col_sum = tilt_col(get_col(&lines, i));
        println!("Col: {}, Sum: {}\n\n", i, col_sum);
        sum += col_sum;
    }
    // sum += tilt_col(get_col(&lines, 0));

    println!("Day 14, Part 1: {}", sum);
}
