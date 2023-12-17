use std::collections::HashMap;

use crate::file_reader::file_reader;

static MULTIPLE: usize = 17;
static REMAINDER: usize = 256;

fn get_ascii_value(c: char) -> usize {
    c as usize
}

fn get_hashmap_value(sequence: &str) -> usize {
    let mut sum = 0;

    for c in sequence.chars().into_iter() {
        let val = get_ascii_value(c);
        sum += val;
        sum = sum * MULTIPLE;
        sum = sum % REMAINDER;
    }

    println!("Sequence: {}, Sum: {}", sequence, sum);

    sum
}

/** `-` as the last character denotes removing */
fn parse_operation(sequence: &str) -> (String, Option<u32>, bool) {
    // Split at "="
    let mut split: Vec<&str> = sequence.split('=').collect();

    // If there is only one item, then we are removing a lens
    if split.len() == 1 {
        let mut split = sequence.split('-');

        let label = split.next().unwrap().to_string();
        let is_add = false;

        (label, None, is_add)
    } else {
        let is_add = true;

        let label = split.remove(0).to_string();
        let value = split.remove(0).parse::<u32>().unwrap();

        (label, Some(value), is_add)
    }
}

fn place_lens(
    map: &mut HashMap<String, usize>,
    boxes: &mut Vec<Vec<(String, u32)>>,
    sequence: &str,
) {
    let sequence = sequence.to_string();

    let (label, focal_strength, is_add) = parse_operation(&sequence);

    let hash_value = get_hashmap_value(&label);

    if is_add {
        // Does the hashmap already contain the label?
        if map.contains_key(&label) {
            let box_num = map.get(&label).unwrap();

            // Get the box
            let box_container = boxes.get_mut(*box_num).unwrap();

            if let Some(i) = box_container.iter_mut().find(|(l, _)| l == &label) {
                let (_, strength) = i;
                *strength = focal_strength.unwrap();
            } else {
                box_container.push((label, focal_strength.unwrap()));
            }
        } else {
            map.insert(label.clone(), hash_value);
            boxes[hash_value].push((label, focal_strength.unwrap()));
        }
    } else {
        let box_num = map.get(&label);

        if box_num.is_none() {
            return;
        }

        let box_num = box_num.unwrap();

        let box_container = boxes.get_mut(*box_num).unwrap();

        // Delete the lens from the box
        box_container.retain(|(l, _)| l != &label);
    }
}

fn calculate_box_sum(lens_box: &Vec<(String, u32)>, box_num: &u32) -> u32 {
    if lens_box.is_empty() {
        return 0;
    }

    let mut sum = 0;

    for (i, (_, strength)) in lens_box.iter().enumerate() {
        let slot: u32 = i as u32 + 1;

        sum += box_num * slot * strength;
    }

    sum
}

pub fn solve() {
    let input = file_reader::read_as_vec("inputs/day15.txt");

    // There is only one line in the input file, so we can unwrap it and split it by commas
    let lenses: Vec<&str> = input.get(0).unwrap().split(',').collect();

    // Setup a hashmap of string/usize pairs
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut vec: Vec<Vec<(String, u32)>> = vec![vec![]; 256];

    for lens in lenses {
        place_lens(&mut map, &mut vec, lens);
    }

    println!("________________________________________");
    println!("Finished placing lenses");

    let mut sum = 0;

    for (i, lens_box) in vec.iter().enumerate() {
        let box_num = i as u32 + 1;

        sum += calculate_box_sum(lens_box, &box_num);
    }

    println!("Day 15, Part 2: {}", sum);
}
