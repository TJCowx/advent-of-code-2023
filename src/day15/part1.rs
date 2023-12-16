use crate::file_reader::file_reader;

static MULTIPLE: u32 = 17;
static REMAINDER: u32 = 256;

fn get_ascii_value(c: char) -> u32 {
    c as u32
}

fn get_sequence_sum(sequence: &str) -> u32 {
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

pub fn solve() {
    let input = file_reader::read_as_vec("inputs/day15.txt");

    // There is only one line in the input file, so we can unwrap it and split it by commas
    let sequence: Vec<&str> = input.get(0).unwrap().split(',').collect();

    let mut sum = 0;

    for s in sequence {
        sum += get_sequence_sum(s);
    }

    println!("Sum: {}", sum);
}
