/**
 * Assumptions:
 *          - There is no number greater than 3 digits
 */
use crate::file_reader::file_reader;

fn get_num_rev(line: &String, start: usize) -> i32 {
    let mut num = String::new();
    let mut i = start;

    while line.chars().nth(i).unwrap().is_numeric() {
        num.insert_str(0, &line.chars().nth(i).unwrap().to_string());
        if i == 0 {
            break;
        }
        i -= 1;
    }

    num.parse().unwrap()
}

fn get_num(line: &String, start: usize) -> i32 {
    let mut num = String::new();
    let mut i = start;

    while line.chars().nth(i).unwrap().is_numeric() {
        num.push(line.chars().nth(i).unwrap());
        if i == line.len() - 1 {
            break;
        }
        i += 1;
    }
    // Get the number in the right order and parse as an i32
    let num: i32 = num.parse().unwrap();

    num
}

fn get_num_both_dir(line: &String, start: usize) -> i32 {
    let mut num = String::new();
    let mut i = start;

    while line.chars().nth(i).unwrap().is_numeric() {
        num.insert_str(0, &line.chars().nth(i).unwrap().to_string());
        if i == 0 {
            break;
        }
        i -= 1;
    }

    i = start + 1;

    while line.chars().nth(i).unwrap().is_numeric() {
        num.push(line.chars().nth(i).unwrap());
        if i == line.len() - 1 {
            break;
        }
        i += 1;
    }

    // Get the number in the right order and parse as an i32
    let num: i32 = num.parse().unwrap();

    num
}

fn check_touching_nums(lines: &Vec<String>, gear_y: &usize, gear_x: &usize) -> i32 {
    let mut nums = Vec::new();

    // Check left
    if *gear_x > 0 {
        let left_char = lines[*gear_y].chars().nth(*gear_x - 1).unwrap();

        if left_char.is_numeric() {
            let line = &lines[*gear_y];
            nums.push(get_num_rev(&line, *gear_x - 1));
        }
    }

    // Check right
    if *gear_x < lines[*gear_y].len() - 1 {
        let line = &lines[*gear_y];
        let right_char = line.chars().nth(*gear_x + 1).unwrap();

        if right_char.is_numeric() {
            nums.push(get_num(&line, *gear_x + 1));
        }
    }

    // Check above
    if *gear_y > 0 {
        let line = &lines[*gear_y - 1];
        let top_left = line.chars().nth(*gear_x - 1).unwrap();
        let middle = line.chars().nth(*gear_x).unwrap();
        let top_right = line.chars().nth(*gear_x + 1).unwrap();

        if top_left.is_numeric() && middle.is_numeric() && top_right.is_numeric() {
            // make a number from combining these 3
            let mut num = String::new();
            num.push(top_left);
            num.push(middle);
            num.push(top_right);

            let num: i32 = num.parse().unwrap();

            nums.push(num);
        } else if top_left.is_numeric() && top_right.is_numeric() {
            nums.push(get_num_rev(&line, *gear_x - 1));
            nums.push(get_num(&line, *gear_x + 1));
        } else if top_left.is_numeric() {
            nums.push(get_num_both_dir(&line, *gear_x - 1));
        } else if top_right.is_numeric() {
            nums.push(get_num_both_dir(&line, *gear_x + 1));
        }
    }

    // Check below
    if *gear_y < lines.len() - 1 {
        let line = &lines[*gear_y + 1];
        let bottom_left = line.chars().nth(*gear_x - 1).unwrap();
        let middle = line.chars().nth(*gear_x).unwrap();
        let bottom_right = line.chars().nth(*gear_x + 1).unwrap();

        if bottom_left.is_numeric() && middle.is_numeric() && bottom_right.is_numeric() {
            // make a number from combining these 3
            let mut num = String::new();
            num.push(bottom_left);
            num.push(middle);
            num.push(bottom_right);

            let num: i32 = num.parse().unwrap();

            nums.push(num);
        } else if bottom_left.is_numeric() && bottom_right.is_numeric() {
            nums.push(get_num_rev(&line, *gear_x - 1));
            nums.push(get_num(&line, *gear_x + 1));
        } else if bottom_left.is_numeric() {
            nums.push(get_num_both_dir(&line, *gear_x - 1));
        } else if bottom_right.is_numeric() {
            nums.push(get_num_both_dir(&line, *gear_x + 1));
        }
    }

    if nums.len() == 2 {
        println!("Multiplying {:#?}", nums);
        nums[0] * nums[1]
    } else {
        0
    }
}

pub fn solve() {
    let mut lines = file_reader::read_as_vec("inputs/day03.txt");

    let mut sum = 0;

    for i in 0..lines.len() {
        while let Some(gear) = lines[i].find('*') {
            println!("Found a gear @ {}, {}", gear, i);
            // Replace the gear with a "V"
            lines[i].replace_range(gear..gear + 1, "V");
            sum += check_touching_nums(&lines, &i, &gear);
        }
    }

    println!("Day 03, Part 2: {}", sum);
}
