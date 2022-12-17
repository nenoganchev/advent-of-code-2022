use std::env;

mod part1;
mod part2;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 2 {
        println!("Error: Missing arguments");
        return;
    }
    let part = &args[0];
    let filename = &args[1];

    match part.as_str() {
        "part1" => part1::solve(filename),
        "part2" => part2::solve(filename),
        _ => println!("Error: specify which part to solve in first argument"),
    }
}
