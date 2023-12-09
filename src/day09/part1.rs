use crate::file_reader::file_reader;

fn is_last(nums: &Vec<i32>) -> bool {
    // All numbers are the same
    nums.iter().all(|&x| x == 0)
}

fn get_diff_array(nums: &Vec<i32>) -> Vec<i32> {
    let mut diffs = vec![];
    for i in 0..nums.len() - 1 {
        diffs.push(nums[i + 1] - nums[i]);
    }
    diffs
}

fn gen_diff_array(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut last_arr: Vec<i32> = nums;
    let mut diffs = vec![last_arr.clone()];

    // Generate the diff arrays
    while !is_last(&last_arr) {
        let new_diffs = get_diff_array(&last_arr);

        last_arr = new_diffs.clone();
        diffs.push(new_diffs);
    }

    diffs
}

fn solve_sequence(nums: Vec<i32>) -> i32 {
    let mut diffs = gen_diff_array(nums);
    let mut sum = 0;

    for diff in diffs.iter_mut().rev() {
        sum += diff.last().unwrap();

        diff.push(sum);
    }

    sum
}

fn get_nums_from_str(line: &String) -> Vec<i32> {
    let mut nums = vec![];
    for num in line.split_whitespace() {
        nums.push(num.parse::<i32>().unwrap());
    }
    nums
}

pub fn parse_sequences(lines: &Vec<String>) -> i32 {
    lines
        .iter()
        .map(|line| {
            let solved = solve_sequence(get_nums_from_str(line));

            println!("\nSOLVED - {} = {}", &line, solved);
            solved
        })
        .sum()
}

pub fn solve() {
    let lines = file_reader::read_as_vec("inputs/day09.txt");

    let res = parse_sequences(&lines);

    println!("Day 09, Part 1: {}", res);
}
