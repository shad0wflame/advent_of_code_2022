use std::fs::read_to_string;
use std::path::PathBuf;

pub enum Hand {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

pub enum Result {
    LOSE = 0,
    DRAW = 3,
    WIN = 6,
}

impl Hand {
    fn fight(&self, enemy: &Hand) -> u32 {
        match (self, enemy) {
            (Hand::ROCK, Hand::ROCK) => Hand::ROCK as u32 + Result::DRAW as u32,
            (Hand::ROCK, Hand::PAPER) => Hand::ROCK as u32 + Result::LOSE as u32,
            (Hand::ROCK, Hand::SCISSORS) => Hand::ROCK as u32 + Result::WIN as u32,
            (Hand::PAPER, Hand::PAPER) => Hand::PAPER as u32 + Result::DRAW as u32,
            (Hand::PAPER, Hand::ROCK) => Hand::PAPER as u32 + Result::WIN as u32,
            (Hand::PAPER, Hand::SCISSORS) => Hand::PAPER as u32 + Result::LOSE as u32,
            (Hand::SCISSORS, Hand::SCISSORS) => Hand::SCISSORS as u32 + Result::DRAW as u32,
            (Hand::SCISSORS, Hand::ROCK) => Hand::SCISSORS as u32 + Result::LOSE as u32,
            (Hand::SCISSORS, Hand::PAPER) => Hand::SCISSORS as u32 + Result::WIN as u32,
        }
    }
}

fn open_input() -> std::io::Result<String> {
    let current_dir = std::env::current_dir()?;

    read_to_string(PathBuf::from(format!(
        "{}/day_2/{}",
        current_dir.display(),
        "input.txt"
    )))
}

fn parse(hand: &str) -> Option<Hand> {
    match hand {
        "A" | "X" => Some(Hand::ROCK),
        "B" | "Y" => Some(Hand::PAPER),
        "C" | "Z" => Some(Hand::SCISSORS),
        _ => None,
    }
}

pub fn first_solution() -> u32 {
    let rounds = open_input().expect("Unable to open input file.");

    // Get the sum of all results
    rounds
        .lines()
        .map(|round| {
            // Convert text to Hand
            round
                .split_whitespace()
                .map(parse)
                .collect::<Vec<Option<Hand>>>()
        })
        .map(|round| {
            // Play the game and get a result
            round[1]
                .as_ref()
                .unwrap()
                .fight(&round[0].as_ref().unwrap())
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::Hand;

    #[test]
    fn test_a_vs_y() {
        assert_eq!(Hand::PAPER.fight(&Hand::ROCK), 8);
    }

    #[test]
    fn test_b_vs_x() {
        assert_eq!(Hand::ROCK.fight(&Hand::PAPER), 1);
    }

    #[test]
    fn test_c_vs_z() {
        assert_eq!(Hand::SCISSORS.fight(&Hand::SCISSORS), 6);
    }
}
