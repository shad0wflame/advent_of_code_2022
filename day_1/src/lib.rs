use std::fs::read_to_string;
use std::path::PathBuf;

fn open_input() -> std::io::Result<String> {
    let current_dir = std::env::current_dir()?;

    read_to_string(PathBuf::from(format!(
        "{}/day_1/{}",
        current_dir.display(),
        "input.txt"
    )))
}

fn parse(text: &String) -> Vec<usize> {
    text.split("\n\n").map(|calories_text| {
        calories_text
            .split_whitespace()
            .map(|calorie| {
                calorie
                    .parse::<usize>()
                    .unwrap_or(0)
            })
            .sum::<usize>()
    }).collect::<Vec<usize>>()
}

fn get_total_calories_of_elves() -> Vec<usize> {
    let text = open_input().expect("Failed to open input file.");
    parse(&text)
}

pub fn first_solution() -> usize {
    let sum_of_calories = get_total_calories_of_elves();

    sum_of_calories.into_iter().max().unwrap_or(0)
}

pub fn second_solution() -> usize {
    let mut sum_of_calories = get_total_calories_of_elves();

    sum_of_calories.sort();
    sum_of_calories.reverse();

    sum_of_calories.iter().take(3).sum::<usize>()
}
