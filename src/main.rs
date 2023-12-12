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
        _ => error_exit("This day has not been implemented yet!"),
    }

    println!("Time taken: {}ms", start.elapsed().as_millis());
}
