use std::{io, io::prelude::*};
use itertools::Itertools;


fn part1(file_contents: &Vec<(String, String)>) {
    let mut score = 0;
    for (opp, self_) in file_contents {
        match opp.as_str() {
            "A" => match self_.as_str() {
                "X" => score += 4,
                "Y" => score += 8,
                "Z" => score += 3,
                _ => continue,
            },
            "B" => match self_.as_str() {
                "X" => score += 1,
                "Y" => score += 5,
                "Z" => score += 9,
                _ => continue,
            },
            "C" => match self_.as_str() {
                "X" => score += 7,
                "Y" => score += 2,
                "Z" => score += 6,
                _ => continue,
            },
            _ => continue,
        }
    }

    println!("{}", score);
}

enum ChoiceScores {
    ROCK = 1,
    PAPER,
    SCISSORS
}

enum ResultScores {
    LOSE = 0,
    DRAW = 3,
    WIN = 6
}

fn part2(file_contents: &Vec<(String, String)>) {
    let mut score = 0;
    for (opp, self_) in file_contents {
        match opp.as_str() {
            "A" => match self_.as_str() { // Rock
                "X" => score += ResultScores::LOSE as u32 + ChoiceScores::SCISSORS as u32, // Lose / Scissors
                "Y" => score += ResultScores::DRAW as u32 + ChoiceScores::ROCK as u32, // Draw // Rock
                "Z" => score += ResultScores::WIN as u32 + ChoiceScores::PAPER as u32, // Win // Paper
                _ => continue,
            },
            "B" => match self_.as_str() { // Paper
                "X" => score += ResultScores::LOSE as u32 + ChoiceScores::ROCK as u32, // Lose / Rock
                "Y" => score += ResultScores::DRAW as u32 + ChoiceScores::PAPER as u32, // Draw / Paper
                "Z" => score += ResultScores::WIN as u32 + ChoiceScores::SCISSORS as u32, // Win / Scissors
                _ => continue,
            },
            "C" => match self_.as_str() { // Scissors
                "X" => score += ResultScores::LOSE as u32 + ChoiceScores::PAPER as u32, // Lose // Paper
                "Y" => score += ResultScores::DRAW as u32 + ChoiceScores::SCISSORS as u32, // Draw // Scissors
                "Z" => score += ResultScores::WIN as u32 + ChoiceScores::ROCK as u32, // Win // Rock
                _ => continue,
            },
            _ => continue,
        }
        println!("{}", score)
    }

    println!("{}", score);
}

fn main() {
    let file_contents: Vec<(String, String)> = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .map(|y| y.split(" ")
            .map(|x| x.to_string()).collect_tuple::<(String, String)>().unwrap()
        )
        .collect(); 
    // for line in stdin.lock().lines().collect::Vec<String>>() {
    //     file_contents.push(line.split(" ").collect_tuple().unwrap());
    // }

    part1(&file_contents);
    part2(&file_contents);
}