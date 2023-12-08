use regex::Regex;

use crate::file_reader::file_reader;

// X, Y
#[derive(Debug, PartialEq)]
struct Coord(i32, i32);
// Val, Start, End
#[derive(Debug)]
struct Pos<'a>(&'a str, Coord, Coord);

fn get_nums(lines: &Vec<String>) -> Vec<Pos> {
    let mut num_positions: Vec<Pos> = Vec::new();
    // Regex to get numbers
    let reg = Regex::new(r"\d+").unwrap();
    for (x, line) in lines.iter().enumerate() {
        for cap in reg.find_iter(line) {
            let start_coord = Coord(cap.start() as i32, x as i32);
            let end_coord = Coord(cap.end() as i32 - 1, x as i32);
            num_positions.push(Pos(cap.as_str(), start_coord, end_coord));
        }
    }

    num_positions
}

fn get_gears(lines: &Vec<String>) -> Vec<Coord> {
    let mut gears: Vec<Coord> = Vec::new();
    for (x, line) in lines.iter().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '*' {
                gears.push(Coord(y as i32, x as i32));
            }
        }
    }

    gears
}

fn get_gear_ratio(gears: &Vec<Coord>, nums: &Vec<Pos>, max_y: usize, max_x: usize) -> i32 {
    let mut gear_ratio = 0;

    for gear in gears {
        println!("\n\nGear: {:?}", gear);
        println!("______________");
        let neighbors = vec![
            Coord(gear.0 - 1, gear.1 - 1),
            Coord(gear.0, gear.1 - 1),
            Coord(gear.0 + 1, gear.1 - 1),
            Coord(gear.0 - 1, gear.1),
            Coord(gear.0 + 1, gear.1),
            Coord(gear.0 - 1, gear.1 + 1),
            Coord(gear.0, gear.1 + 1),
            Coord(gear.0 + 1, gear.1 + 1),
        ]
        .into_iter()
        .filter(|Coord(x, y)| *x >= 0 && *y >= 0 && *x < max_x as i32 && *y < max_y as i32)
        .collect::<Vec<Coord>>();

        println!("Gear: {:?}", gear);
        println!("Neighbors: {:?}", neighbors);

        let mut ratios = Vec::new();

        // Loop through the numbers and check if they are in the neighbors
        for num in nums {
            // Check if the number is in the neighbors
            for neighbor in &neighbors {
                // The y of the neighbour is the same as the y of the number
                let y_match = neighbor.1 == num.1 .1;
                // The x of the neighbour is between the start and end of the number inclusive
                let x_match = neighbor.0 >= num.1 .0 && neighbor.0 <= num.2 .0;

                if y_match && x_match {
                    println!("Num: {:?}", num);
                    println!("Neighbor: {:?}", neighbor);
                    ratios.push(num.0.parse::<i32>().unwrap());
                    break;
                }
            }
        }

        println!("!!!!Ratios: {:?}", ratios);
        if ratios.len() == 2 {
            println!("!!!!Ratios: {:?}", ratios);
            gear_ratio += ratios[0] * ratios[1];
        }
    }

    gear_ratio
}

pub fn solve() {
    let lines = file_reader::read_as_vec("inputs/day03.txt");

    let num_positions = get_nums(&lines);
    let gears = get_gears(&lines);

    let ratio = get_gear_ratio(&gears, &num_positions, lines.len(), lines[0].len());

    println!("Gear ratio: {}", ratio);
}
