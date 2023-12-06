// use crate::file_reader::file_reader;

/* fn make_num(chars: &mut Vec<char>, x: usize) -> i32 {
    // Go back in x until not a number and that is first
    let mut first = x;
    while first > 0 && chars[first as usize].is_numeric() {
        first -= 1;
    }

    let mut last = x;
    while last < chars.len() as usize && chars[last as usize].is_numeric() {
        last += 1;
    }

    // Get the chars and parse into a number, replace chars with a "V"
    let num: String = chars[first + 1..last].iter().collect();
    let num: i32 = num.parse().unwrap();
    for i in first..=last {
        chars[i] = 'V';
    }

    num
}

fn get_touching_nums(lines: &mut Vec<String>, x: usize, y: usize) -> Vec<i32> {
    let mut nums = Vec::new();

    // Check left
    if x > 0 {
        let chars: &mut Vec<char> = &mut lines[y].chars().into_iter().collect();
        let prev_char = chars.nth(x - 1).unwrap();
        if prev_char.is_numeric() {
            nums.push(make_num(chars, x - 1));
            lines[y] = chars.into_iter().collect();
        }
    }

    // Check right
    if x < lines[y].len() - 1 {
        let chars = &mut lines[y].chars().into_iter().collect();
        let next_char = chars.nth(x + 1).unwrap();
        if next_char.is_numeric() {
            nums.push(make_num(chars, x + 1));
            lines[y] = chars.into_iter().collect();
        }
    }

    // Check above
    if y > 0 {}

    // Check below
    if y < lines.len() - 1 {}

    nums
} */

pub fn solve() {
    /* let mut lines = file_reader::read_as_vec("inputs/day03-test.txt");

    let mut sum = 0;

    for i in 0..lines.len() {
        // Now we have the line, we need to search for '*'s
        let line = &lines[i];
        for j in 0..line.len() {
            if line.chars().nth(j).unwrap() == '*' {
                println!("Found * at line: {}, character: {}", i, j);
                let touching_nums = get_touching_nums(&mut lines, j, i);
                println!("Touching nums: {:?}", touching_nums);
            }
        }
    } */

    println!("Day 03, Part 2");
}
