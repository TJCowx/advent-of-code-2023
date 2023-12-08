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

fn get_starting_points(direction_map: &HashMap<String, Directions>) -> Vec<String> {
    direction_map
        .keys()
        .filter(|key| key.chars().last().unwrap() == 'A')
        .map(|key| key.to_string())
        .collect()
}

fn map_directions(direction_map: &mut HashMap<String, Directions>, lines: Vec<String>) {
    let reg = Regex::new(r"^([A-Z0-9]{3}).*?\(([A-Z0-9]{3}),\s([A-Z0-9]{3})").unwrap();

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
}

fn steps_taken(
    instructions: &Vec<char>,
    direction_map: &HashMap<String, Directions>,
    start: &String,
) -> usize {
    let mut current_label = start.to_string();
    let mut steps_taken = 0;
    let mut i = 0;

    while current_label.chars().last().unwrap() != 'Z' {
        steps_taken += 1;
        let current_direction = direction_map.get(&current_label).unwrap();

        let instruction = instructions[i];

        match instruction {
            'L' => current_label = current_direction.left.to_string(),
            'R' => current_label = current_direction.right.to_string(),
            _ => panic!("Invalid instruction"),
        }

        if i == instructions.len() - 1 {
            i = 0;
        } else {
            i += 1;
        }
    }

    steps_taken
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

fn lcm(a: usize, b: usize) -> usize {
    return a * b / gcd(a, b);
}

pub fn solve() {
    let mut lines = file_reader::read_as_vec("inputs/day08.txt").into_iter();

    let instructions = get_instructions(lines.next().unwrap());

    lines.next(); // Remove the empty line

    let mut direction_map: HashMap<String, Directions> = HashMap::new();

    map_directions(&mut direction_map, lines.clone().collect());

    let starting_points = get_starting_points(&direction_map);

    println!("Finding paths...");

    let lcm = starting_points
        .iter()
        .map(|start| steps_taken(&instructions, &direction_map, start))
        .fold(1, |acc, x| lcm(acc, x));

    println!("Day 08, Part 2: {}", lcm);
}
