use std::collections::HashMap;

use crate::file_reader::file_reader;

#[derive(Debug)]
struct ValRange {
    pub start: u64,
    pub end: u64,
}

#[derive(Debug)]
struct ConversionMap {
    pub source_range_start: u64,
    pub source_range_end: u64,
    pub target_range_start: u64,
}

fn get_seeds(line: String) -> Vec<ValRange> {
    let nums: Vec<u64> = line
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut ranges = vec![];
    for chunk in nums.chunks(2) {
        println!("Chunk: {:#?}", chunk);
        ranges.push(ValRange {
            start: chunk[0],
            end: chunk[0] + chunk[1] - 1, // Include the start
        });
    }

    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    ranges
}

fn process_maps(lines: &Vec<String>) -> HashMap<usize, Vec<ConversionMap>> {
    let mut map = HashMap::new();

    let mut adding_to_map = false;
    for line in lines.iter() {
        // We're done iterating
        if line.is_empty() {
            println!("Done inserting");
            println!("______________________");
            adding_to_map = false;
            continue;
        }

        // If we're not adding to the map, start a new map entry
        if !adding_to_map {
            println!("Inserting to {}", line);
            adding_to_map = true;
            map.insert(map.len(), vec![]);
            continue;
        }

        let key = map.len() - 1;

        // Parse the line into the 3 numbers
        let nums = line
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        // 0 is destination start, 1 is source start, 2 is length of range
        // Add the conversion map to the map
        let conversion_map = ConversionMap {
            source_range_start: nums[1],
            source_range_end: nums[1] + nums[2] - 1,
            target_range_start: nums[0],
        };

        let map_entry = map.get_mut(&key).unwrap();

        println!("Inserting into map ({}): {:#?}", key, conversion_map);

        map_entry.push(conversion_map);
    }

    map
}

// This is a brute force solution that takes ~1.8 billion iterations
// Takes my laptop > 3 hours to run before I quit running it
// Answer: 20358599
pub fn solve() {
    let mut lines = file_reader::read_as_vec("inputs/day05.txt");

    let mut lowest: u64 = u64::MAX;

    let seed_ranges = get_seeds(lines.remove(0));

    let maps = process_maps(&lines);

    let mut keys: Vec<&usize> = maps.keys().collect();
    keys.sort();

    for seed_range in seed_ranges {
        println!("Processing Seed: {:#?}", seed_range);
        for mut current in seed_range.start..seed_range.end {
            for key in keys.iter() {
                let map = maps.get(key).unwrap();

                for conversion_map in map.iter() {
                    // If the current number is in the source range
                    if current >= conversion_map.source_range_start
                        && current <= conversion_map.source_range_end
                    {
                        // TODO: Handle the update to the next number
                        let diff = current - conversion_map.source_range_start;
                        current = conversion_map.target_range_start + diff;
                        break;
                    }
                }
            }

            if current < lowest {
                println!("New Lowest: {} - Was: {}", current, lowest);
                lowest = current;
            }
        }
        println!("\n______________________\n");
    }

    println!("Day 05, Part 1 Result: {}", lowest);
}
