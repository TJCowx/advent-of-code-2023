use crate::file_reader::file_reader;

fn get_num_from_string(s: &str) -> u64 {
    s.chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

pub fn solve() {
    let mut lines = file_reader::read_as_vec("inputs/day06.txt");

    let time_line = lines.remove(0);
    let distance_line = lines.remove(0);

    let record = get_num_from_string(&time_line);
    let distance = get_num_from_string(&distance_line);

    println!("Record Time: {:#?}", record);
    println!("Distance: {:#?}", distance);

    let mut times_beaten = 0;

    // Lets brute force this
    for hold_time in 1..record {
        let time_remaining = record - hold_time;

        let distance_traveled = hold_time * time_remaining;

        if distance_traveled > distance {
            times_beaten += 1;
        }
    }

    println!("\n\nRESULT: {}", times_beaten);
}
