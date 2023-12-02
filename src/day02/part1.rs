use crate::file_reader::file_reader;

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;
// only 12 red cubes, 13 green cubes, and 14 blue cubes

fn get_game_num(game_label: &str) -> i32 {
    game_label
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}

pub fn solve() {
    let lines = file_reader::read_as_vec("inputs/day02.txt");

    let mut total = 0;

    for line in lines.iter() {
        let mut game_possible = true;
        let mut split_line = line.split(":");
        let game_label = split_line.next().unwrap().trim();
        let rounds = split_line.next().unwrap().split(";");

        'round_loop: for round in rounds {
            let rolls = round.split(",");

            for roll in rolls {
                let mut split_roll = roll.trim().split(" ");

                let count = split_roll.next().unwrap().parse::<i32>().unwrap();

                let color = split_roll.next().unwrap();

                let max = match color {
                    "red" => MAX_RED,
                    "green" => MAX_GREEN,
                    "blue" => MAX_BLUE,
                    _ => 100,
                };

                if count > max {
                    game_possible = false;

                    break 'round_loop;
                }
            }
        }

        if game_possible {
            let game_num = get_game_num(game_label);
            total += game_num;
        }
    }

    println!("Day 02 Part 1: {}", total);
}
