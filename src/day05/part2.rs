/* #[derive(Debug)]
struct ConversionMap {
    pub source_range_start: u64,
    pub source_range_end: u64,
    pub target_range_start: u64,
    pub target_range_end: u64,
}

#[derive(Debug)]
struct ValRange {
    pub start: u64,
    pub end: u64,
}

fn get_seeds(line: String) -> Vec<ValRange> {
    let nums: Vec<u64> = line
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut ranges = vec![];
    for window in nums.windows(2) {
        ranges.push(ValRange {
            start: window[0],
            end: window[0] + window[1],
        });
    }

    ranges
}

fn process_maps(lines: &Vec<String>) -> HashMap<usize, Vec<ConversionMap>> {
    let mut map = HashMap::new();

    let mut adding_to_map = false;
    for line in lines.iter() {
        // We're done iterating
        if line.is_empty() {
            adding_to_map = false;
            continue;
        }

        // If we're not adding to the map, start a new map entry
        if !adding_to_map {
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
            target_range_end: nums[0] + nums[2] - 1,
        };

        let map_entry = map.get_mut(&key).unwrap();

        map_entry.push(conversion_map);
    }

    map
}

fn readable_map_name(key: &usize) -> String {
    match key {
        0 => "Seed to Soil".to_string(),
        1 => "Soil to Fertilizer".to_string(),
        2 => "Fertilizer to Water".to_string(),
        3 => "Water to Light".to_string(),
        4 => "Light to Temp".to_string(),
        5 => "Temp to Humidity".to_string(),
        6 => "Humidity to Location".to_string(),
        _ => "Unknown".to_string(),
    }
} */

// TODO: I actually want to solve this
pub fn solve() {
    println!("Day 5 Part 2");
    /*     let mut lines = file_reader::read_as_vec("inputs/day05.txt");

    let mut lowest: u64 = u64::MAX;

    let seeds = get_seeds(lines.remove(0));

    println!("Found Seeds");
    println!("______________________");
    println!("{:#?}", seeds);

    println!("Processing maps...");
    let maps = process_maps(&lines);
    println!("Done processing maps"); */
}
