use std::env;
use std::fs::File;
use std::io::BufReader;

mod day01;
mod day02;
mod day03;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 3 {
        println!("Error: Missing arguments");
        return;
    }
    let day = &args[0];
    let part = &args[1];
    let input_filename = &args[2];

    let input_file =
        match File::open(input_filename) {
            Ok(file) => file,
            Err(error) => {
                println!("Error: Cannot open file `{}` ({})", input_filename, error);
                return;
            },
        };
    let buffered_input = BufReader::new(input_file);

    match (day.as_str(), part.as_str()) {
        ("day1", "part1") => day01::part1::solve(input_filename),
        ("day1", "part2") => day01::part2::solve(input_filename),
        ("day2", "part1") => day02::part1::solve(input_filename),
        ("day2", "part2") => day02::part2::solve(input_filename),
        ("day3", "part1") => day03::part1::solve(buffered_input),
        ("day3", "part2") => day03::part2::solve(buffered_input),
        _ => println!("Error: specify which part to solve in first argument"),
    }
}
