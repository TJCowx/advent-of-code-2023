mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
pub mod file_reader;

fn error_exit(message: &str) {
    eprintln!("{}", message);
    std::process::exit(1);
}

fn main() {
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
            _ => error_exit("There is only 2 parts"),
        },
        _ => error_exit("This day has not been implemented yet!"),
    }
}
