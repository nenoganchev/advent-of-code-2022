use std::env;

mod day01;
mod day02;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 3 {
        println!("Error: Missing arguments");
        return;
    }
    let day = &args[0];
    let part = &args[1];
    let filename = &args[2];

    match (day.as_str(), part.as_str()) {
        ("day1", "part1") => day01::part1::solve(filename),
        ("day1", "part2") => day01::part2::solve(filename),
        ("day2", "part1") => day02::part1::solve(filename),
        ("day2", "part2") => day02::part2::solve(filename),
        _ => println!("Error: specify which part to solve in first argument"),
    }
}
