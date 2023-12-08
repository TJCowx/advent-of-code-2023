use regex::Regex;
use std::collections::HashMap;

use crate::file_reader::file_reader;

#[derive(Debug)]
struct Directions {
    left: String,
    right: String,
}

fn get_instructions(line: String) -> Vec<char> {
    let mut instructions: Vec<char> = Vec::new();
    for c in line.chars() {
        instructions.push(c);
    }
    instructions
}

pub fn solve() {
    let mut lines = file_reader::read_as_vec("inputs/day08.txt").into_iter();

    let instructions = get_instructions(lines.next().unwrap());

    lines.next(); // Remove the empty line

    let mut direction_map: HashMap<String, Directions> = HashMap::new();

    let reg = Regex::new(r"^([A-Z]{3}).*?\(([A-Z]{3}),\s([A-Z]{3})").unwrap();

    let mut steps_taken = 0;

    for line in lines {
        let (_, [label, left, right]) = reg.captures(&line).unwrap().extract();

        direction_map.insert(
            label.to_string(),
            Directions {
                left: left.to_string(),
                right: right.to_string(),
            },
        );
    }

    println!("Mapped Directions");

    println!("Finding paths...");

    let mut current_label = String::from("AAA");

    let mut instruction_i = 0;

    while current_label != "ZZZ" {
        steps_taken += 1;
        println!("{}: {}", steps_taken, current_label);

        let current_direction = direction_map.get(&current_label).unwrap();

        let instruction = instructions[instruction_i];

        match instruction {
            'L' => current_label = current_direction.left.to_string(),
            'R' => current_label = current_direction.right.to_string(),
            _ => panic!("Invalid instruction"),
        }

        if instruction_i == instructions.len() - 1 {
            instruction_i = 0;
        } else {
            instruction_i += 1;
        }
    }

    println!("Day 08, Part 1: {}", steps_taken);
}
