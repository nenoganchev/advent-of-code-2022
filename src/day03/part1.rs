use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

pub fn solve(input: BufReader<File>) {
    let mut sum = 0;

    for line in input.lines().map(Result::unwrap) {
        assert!(line.len() % 2 == 0);
        let middle = line.len() / 2;
        let first_half = &line[..middle];
        let second_half = &line[middle..];

        let first_compartment: HashSet<char> = first_half.chars().collect();
        let second_compartment: HashSet<char> = second_half.chars().collect();

        for common in first_compartment.intersection(&second_compartment) {
            sum += item_priority(*common);
        }
    }

    println!("Priorities sum: {}", sum);
}

fn item_priority(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 'a' as u32 + 1,
        'A'..='Z' => item as u32 - 'A' as u32 + 27,
        _ => panic!("Unexpected item `{}`", item),
    }
}
