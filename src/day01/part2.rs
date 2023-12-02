use crate::file_reader::file_reader;

fn process_string(s: String) -> String {
    let mut s_copy = s.clone();
    s_copy = s_copy.replace("one", "o1e");
    s_copy = s_copy.replace("two", "t2o");
    s_copy = s_copy.replace("three", "t3hree");
    s_copy = s_copy.replace("four", "f4ur");
    s_copy = s_copy.replace("five", "f5ve");
    s_copy = s_copy.replace("six", "s6x");
    s_copy = s_copy.replace("seven", "s7ven");
    s_copy = s_copy.replace("eight", "e8ght");
    s_copy = s_copy.replace("nine", "n9ne");

    return s_copy;
}

pub fn part2() {
    let lines = file_reader::read_as_vec("inputs/day01.txt");

    let mut total = 0;

    for line in lines.iter() {
        let mut processed = process_string(line.to_string());
        processed.retain(|c| c.is_numeric());

        let first_char = processed.chars().nth(0).unwrap();
        let last_char = processed.chars().nth(processed.len() - 1).unwrap();

        total += format!("{}{}", first_char, last_char)
            .parse::<i32>()
            .unwrap();

        println!("{}", processed);
    }

    println!("Day 01 Part 2: {}", total);
}
