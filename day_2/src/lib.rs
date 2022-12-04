extern crate core;

use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Hand {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

#[derive(Debug)]
pub enum Result {
    LOSE = 0,
    DRAW = 3,
    WIN = 6,
}

#[derive(Debug)]
pub enum Round {
    HAND(Hand),
    RESULT(Result),
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

impl Result {
    fn deduce_other_hand(&self, hand: &Hand) -> Hand {
        match (self, hand) {
            (Result::WIN, Hand::ROCK) => Hand::PAPER,
            (Result::WIN, Hand::PAPER) => Hand::SCISSORS,
            (Result::WIN, Hand::SCISSORS) => Hand::ROCK,
            (Result::DRAW, Hand::ROCK) => Hand::ROCK,
            (Result::DRAW, Hand::PAPER) => Hand::PAPER,
            (Result::DRAW, Hand::SCISSORS) => Hand::SCISSORS,
            (Result::LOSE, Hand::ROCK) => Hand::SCISSORS,
            (Result::LOSE, Hand::PAPER) => Hand::ROCK,
            (Result::LOSE, Hand::SCISSORS) => Hand::PAPER,
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

fn wrong_parse(hand: &str) -> Option<Hand> {
    match hand {
        "A" | "X" => Some(Hand::ROCK),
        "B" | "Y" => Some(Hand::PAPER),
        "C" | "Z" => Some(Hand::SCISSORS),
        _ => None,
    }
}

fn right_parse(text: &str) -> Option<Round> {
    match text {
        "A" => Some(Round::HAND(Hand::ROCK)),
        "B" => Some(Round::HAND(Hand::PAPER)),
        "C" => Some(Round::HAND(Hand::SCISSORS)),
        "X" => Some(Round::RESULT(Result::LOSE)),
        "Y" => Some(Round::RESULT(Result::DRAW)),
        "Z" => Some(Round::RESULT(Result::WIN)),
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
                .map(wrong_parse)
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

pub fn second_solution() -> u32 {
    let rounds = open_input().expect("Unable to open input file.");

    // Get the sum of all results
    rounds
        .lines()
        .map(|round| {
            // Convert text to Round
            round
                .split_whitespace()
                .map(right_parse)
                .collect::<Vec<Option<Round>>>()
        })
        .map(|round| {
            let enemy_hand = if let Round::HAND(enemy_hand) = round[0].as_ref().unwrap() {
                enemy_hand
            } else {
                panic!()
            };

            let result= if let Round::RESULT(result) = round[1].as_ref().unwrap() {
                result
            } else {
                panic!()
            };

            let own_hand: Hand = result.deduce_other_hand(enemy_hand);

            own_hand.fight(enemy_hand)
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
