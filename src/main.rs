use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
pub mod file_reader;

fn error_exit(message: &str) {
    eprintln!("{}", message);
    std::process::exit(1);
}

fn main() {
    let start = Instant::now();
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        error_exit("Please enter the day and part number (EG: 1.1 for day 2 part 1)");
    }

    let day_part: Vec<&str> = args[1].split('.').collect();
    if day_part.len() < 2 {
        error_exit(
            "Invalid arguments please enter it in day.part format (EG: 1.1 for day 2 part 1)",
        );
    }

    match day_part[0] {
        "1" => match day_part[1] {
            "1" => day01::part1::solve(),
            "2" => day01::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "2" => match day_part[1] {
            "1" => day02::part1::solve(),
            "2" => day02::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "3" => match day_part[1] {
            "1" => day03::part1::solve(),
            "2" => day03::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "4" => match day_part[1] {
            "1" => day04::part1::solve(),
            "2" => day04::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "5" => match day_part[1] {
            "1" => day05::part1::solve(),
            "2" => day05::part2::solve(),
            "2b" => day05::part2b::solve(), // B for brute force 😼
            _ => error_exit("There is only 2 parts"),
        },
        "6" => match day_part[1] {
            "1" => day06::part1::solve(),
            "2" => day06::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "7" => match day_part[1] {
            "1" => day07::part1::solve(),
            "2" => day07::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "8" => match day_part[1] {
            "1" => day08::part1::solve(),
            "2" => day08::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "9" => match day_part[1] {
            "1" => day09::part1::solve(),
            "2" => day09::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "10" => match day_part[1] {
            "1" => day10::part1::solve(),
            "2" => day10::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "11" => match day_part[1] {
            "1" => day11::part1::solve(),
            "2" => day11::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "12" => match day_part[1] {
            "1" => day12::part1::solve(),
            "2" => day12::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "13" => match day_part[1] {
            "1" => day13::part1::solve(),
            "2" => day13::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "14" => match day_part[1] {
            "1" => day14::part1::solve(),
            "2" => day14::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "15" => match day_part[1] {
            "1" => day15::part1::solve(),
            "2" => day15::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "16" => match day_part[1] {
            "1" => day16::part1::solve(),
            "2" => day16::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "17" => match day_part[1] {
            "1" => day17::part1::solve(),
            "2" => day17::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "18" => match day_part[1] {
            "1" => day18::part1::solve(),
            "2" => day18::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "19" => match day_part[1] {
            "1" => day19::part1::solve(),
            "2" => day19::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "20" => match day_part[1] {
            "1" => day20::part1::solve(),
            "2" => day20::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "21" => match day_part[1] {
            "1" => day21::part1::solve(),
            "2" => day21::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "22" => match day_part[1] {
            "1" => day22::part1::solve(),
            "2" => day22::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "23" => match day_part[1] {
            "1" => day23::part1::solve(),
            "2" => day23::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "24" => match day_part[1] {
            "1" => day24::part1::solve(),
            "2" => day24::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        "25" => match day_part[1] {
            "1" => day25::part1::solve(),
            "2" => day25::part2::solve(),
            _ => error_exit("There is only 2 parts"),
        },
        _ => error_exit("This day has not been implemented yet!"),
    }

    println!("Time taken: {}ms", start.elapsed().as_millis());
}
