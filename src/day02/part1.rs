use std::fs;

pub fn solve(input_filename: &str) {
    let input_str = fs::read_to_string(input_filename).unwrap();
    let total_score: i32 =
        input_str.lines()
            .map( |line| Round::from_input_line(line) )
            .map( |round| round.score() )
            .sum();

    println!("Total score: {}", total_score);
}

// private

enum Outcome {
    Win,
    Loss,
    Draw,
}

#[derive(Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

struct Round {
    my_choice: Choice,
    opponent_choice: Choice,
}

impl Choice {
    fn against(self, other: Choice) -> Outcome {
        match (self, other) {
            (Self::Rock, Self::Paper) => Outcome::Loss,
            (Self::Rock, Self::Scissors) => Outcome::Win,
            (Self::Paper, Self::Rock) => Outcome::Win,
            (Self::Paper, Self::Scissors) => Outcome::Loss,
            (Self::Scissors, Self::Rock) => Outcome::Loss,
            (Self::Scissors, Self::Paper) => Outcome::Win,
            _ => Outcome::Draw,
        }
    }
}

impl Round {
    fn from_input_line(line: &str) -> Self {
        let line = line.as_bytes();
        let (opponent_choice, space, my_choice) = (line[0] as char, line[1] as char, line[2] as char);

        let opponent_choice =
            match opponent_choice {
                'A' => Choice::Rock,
                'B' => Choice::Paper,
                'C' => Choice::Scissors,
                _ => panic!("Invalid opponent choice: '{}'", opponent_choice),
            };

        let my_choice =
            match my_choice {
                'X' => Choice::Rock,
                'Y' => Choice::Paper,
                'Z' => Choice::Scissors,
                _ => panic!("Invalid my choice: '{}'", my_choice),
            };

        assert!(space == ' ', "Expected space between choices, got '{}'", space);

        Round {opponent_choice, my_choice}
    }

    fn score(&self) -> i32 {
        let shape_score =
            match self.my_choice {
                Choice::Rock => 1,
                Choice::Paper => 2,
                Choice::Scissors => 3,
            };

        let outcome_score =
            match self.my_choice.against(self.opponent_choice) {
                Outcome::Loss => 0,
                Outcome::Draw => 3,
                Outcome::Win => 6,
            };

        shape_score + outcome_score
    }
}
