use std::fs;

pub fn solve(input_filename: &str) {
    let elfs_calories: Vec<i32> =
        fs::read_to_string(input_filename).unwrap()
            .lines().collect::<Vec<&str>>()
            .split( |line| line.is_empty() )
            .map( |elf_calories| {
                      elf_calories.iter().fold(0, |acc, calories_str| acc + calories_str.parse::<i32>().unwrap() )
                  }).collect();

    println!("Most calories: {}", elfs_calories.iter().max().unwrap());
}
