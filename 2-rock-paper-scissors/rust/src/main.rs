use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

#[derive(PartialEq)]
struct Move {
    shape: Shape,
    score: i32,
    beats: Shape
}

#[derive(PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

enum Outcome {
    Win,
    Lose,
    Draw
}

#[derive(Debug)]
struct ParseMoveError;

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move {shape: Shape::Rock, score: 1, beats: Shape::Scissors}),
            "B" | "Y" => Ok(Move {shape: Shape::Paper, score: 2, beats: Shape::Rock}),
            "C" | "Z" => Ok(Move {shape: Shape::Scissors, score: 3, beats: Shape::Paper}),
            _ => Err(ParseMoveError)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut accum_score = 0;
    for line in reader.lines() {
        if let Ok(round) = line {
            let moves: Vec<Move> = round.split(' ').map(|s| s.parse().unwrap()).collect();
            let my_move = moves.get(1).expect("Failed to get my move");
            let opponent_move = moves.get(0).expect("Failed to get opponent's move");

            let outcome;
            if my_move.beats == opponent_move.shape {
                outcome = Outcome::Win;
            } else if my_move.shape == opponent_move.shape {
                outcome = Outcome::Draw;
            } else {
                outcome = Outcome::Lose;
            }

            let outcome_score = match outcome {
                Outcome::Win => 6,
                Outcome::Draw => 3,
                Outcome::Lose => 0
            };

            let score = my_move.score + outcome_score;
            accum_score += score;
        }
    }

    println!("Part One Answer: {}", accum_score);
}
