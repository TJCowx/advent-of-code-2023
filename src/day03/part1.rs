use crate::file_reader::file_reader;

struct NumInfo {
    number: i32,
    start_index: usize,
    end_index: usize,
}

fn get_num(chars: &mut Vec<char>, index: usize) -> NumInfo {
    let mut num = String::new();
    num.push(chars[index].clone());
    let mut last_index = index;

    for i in index + 1..chars.len() {
        if chars[i].is_numeric() {
            last_index = i;
            num.push(chars[i].clone());
            chars[i] = 'V';
        } else {
            break;
        }
    }

    NumInfo {
        number: num.parse::<i32>().unwrap(),
        start_index: index,
        end_index: last_index,
    }
}

fn is_serial(char: &char) -> bool {
    let eval = !char.is_numeric() && *char != '.';

    eval
}

fn check_for_valid(lines: &Vec<String>, y: usize, num_first_i: usize, num_last_i: usize) -> bool {
    let start_x = match num_first_i {
        0 => 0,
        _ => {
            if num_first_i == lines[y].len() - 1 {
                num_first_i
            } else {
                num_first_i - 1
            }
        }
    };
    let end_x = match num_last_i {
        0 => 0,
        _ => {
            if num_last_i == lines[y].len() - 1 {
                num_last_i
            } else {
                num_last_i + 1
            }
        }
    };

    // Check one index to the left and one index to the right
    if num_first_i > 0 {
        if is_serial(&lines[y].chars().nth(start_x).unwrap()) {
            return true;
        }
    }

    if num_last_i < lines[y].len() - 1 {
        if is_serial(&lines[y].chars().nth(end_x).unwrap()) {
            return true;
        }
    }

    // Check previous line
    if y > 0 {
        let prev_line = &lines[y - 1];
        // Loop from start_x to end_x
        for i in start_x..=end_x {
            if is_serial(&prev_line.chars().nth(i).unwrap()) {
                return true;
            }
        }
    }

    // Check next line
    if y < lines.len() - 1 {
        let next_line = &lines[y + 1];
        // Loop from start_x to end_x
        for i in start_x..=end_x {
            if is_serial(&next_line.chars().nth(i).unwrap()) {
                return true;
            }
        }
    }

    false
}

pub fn solve() {
    let lines = file_reader::read_as_vec("inputs/day03.txt");

    let mut sum = 0;

    for i in 0..lines.len() {
        let mut chars: Vec<char> = lines[i].chars().collect();

        for j in 0..chars.len() {
            if chars[j].is_numeric() {
                let num_info = get_num(&mut chars, j);

                if check_for_valid(&lines, i, num_info.start_index, num_info.end_index) {
                    println!("{} is valid", num_info.number);
                    sum += num_info.number;
                } else {
                    println!("{} is not valid", num_info.number);
                }
            }
        }
    }

    println!("Day 03, Part 1: {}", sum);
}
