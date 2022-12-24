use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

struct Group {
    sack1: HashSet<char>,
    sack2: HashSet<char>,
    sack3: HashSet<char>,
}

pub fn solve(input: BufReader<File>) {
    let mut groups = Vec::new();

    let mut lines_iter = input.lines().map(Result::unwrap);
    while let Some(line1) = lines_iter.next() {
        let line2 = lines_iter.next().unwrap();
        let line3 = lines_iter.next().unwrap();

        groups.push(Group{
            sack1: line1.chars().collect(),
            sack2: line2.chars().collect(),
            sack3: line3.chars().collect(),
        });
    }

    let mut sum = 0;
    for group in groups {
        let common_items = group.sack1.intersection(&group.sack2).copied().collect::<HashSet<_>>()
                                      .intersection(&group.sack3).copied().collect::<Vec<_>>();
        assert!(common_items.len() == 1);

        let common_item = *common_items.first().unwrap();
        sum += item_priority(common_item);
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
