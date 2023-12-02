use crate::file_reader::file_reader;

pub fn solve() {
    let lines = file_reader::read_as_vec("inputs/day02.txt");

    let mut total = 0;

    for line in lines.iter() {
        let mut split_line = line.split(":");
        let rounds = split_line.next().unwrap().split(";");

        let mut red_min = 0;
        let mut green_min = 0;
        let mut blue_min = 0;

        for round in rounds {
            let rolls = round.split(",");

            for roll in rolls {
                let mut split_roll = roll.trim().split(" ");

                let count = split_roll.next().unwrap().parse::<i32>().unwrap();

                let color = split_roll.next().unwrap();

                match color {
                    "red" => {
                        if count > red_min {
                            red_min = count;
                        }
                    }
                    "green" => {
                        if count > green_min {
                            green_min = count;
                        }
                    }
                    "blue" => {
                        if count > blue_min {
                            blue_min = count;
                        }
                    }
                    _ => (),
                }
            }
        }

        total += red_min * green_min * blue_min;
    }

    println!("Day 02 Part 2: {}", total);
}
