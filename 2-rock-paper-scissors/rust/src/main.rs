use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

#[derive(PartialEq, Debug)]
struct Move {
    shape: Shape,
    score: i32,
    beats: Shape,
    loses: Shape
}

#[derive(PartialEq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

#[derive(Debug)]
struct ParseMoveError;

impl Move {
    pub fn new(shape: Shape) -> Self {
        match shape {
            Shape::Rock => Move {
                shape: Shape::Rock,
                score: 1,
                beats: Shape::Scissors,
                loses: Shape::Paper,
            },
            Shape::Paper => Move {
                shape: Shape::Paper,
                score: 2,
                beats: Shape::Rock,
                loses: Shape::Scissors,
            },
            Shape::Scissors => Move {
                shape: Shape::Scissors,
                score: 3,
                beats: Shape::Paper,
                loses: Shape::Rock
            },
        }
    }
}

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::new(Shape::Rock)),
            "B" | "Y" => Ok(Move::new(Shape::Paper)),
            "C" | "Z" => Ok(Move::new(Shape::Scissors)),
            _ => Err(ParseMoveError),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();

    part_one(&lines);
    part_two(&lines);
}

fn part_one(rounds: &Vec<String>) {
    let mut accum_score = 0;
    for round in rounds.iter() {
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

        let score = my_move.score + outcome as i32;
        accum_score += score;
    }

    println!("Part One Answer: {}", accum_score);
}

fn part_two(rounds: &Vec<String>) {
    let mut accum_score = 0;
    for round in rounds.iter() {
        let opponent_move = round.split(' ').nth(0).unwrap().parse::<Move>().unwrap();
        let outcome = match round.split(' ').nth(1).unwrap() {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            _ => Outcome::Win
        };

        let my_move = match outcome {
            Outcome::Win => Move::new(opponent_move.loses),
            Outcome::Draw => Move::new(opponent_move.shape),
            Outcome::Lose => Move::new(opponent_move.beats)
        };

        let score = my_move.score + outcome as i32;
        accum_score += score;
    }

    println!("Part Two Answer: {}", accum_score);
}
