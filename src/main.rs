mod day01;
mod day02;
pub mod file_reader;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Please enter the day and part number (EG: 1.1 for day 2 part 1)");
        std::process::exit(1);
    }

    let day_part: Vec<&str> = args[1].split('.').collect();
    if day_part.len() < 2 {
        eprintln!(
            "Invalid arguments please enter it in day.part format (EG: 1.1 for day 2 part 1)"
        );
        std::process::exit(1);
    }

    match day_part[0] {
        "1" => match day_part[1] {
            "1" => day01::part1::solve(),
            "2" => day01::part2::solve(),
            _ => {
                eprintln!("There is only 2 parts");
                std::process::exit(1);
            }
        },
        "2" => match day_part[1] {
            "1" => day02::part1::solve(),
            "2" => day02::part2::solve(),
            _ => {
                eprintln!("There is only 2 parts");
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("This day has not been implemented yet!");
            std::process::exit(1);
        }
    }
}
