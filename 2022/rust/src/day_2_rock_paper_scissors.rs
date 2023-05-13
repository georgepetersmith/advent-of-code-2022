use std::str::FromStr;

pub fn run_part(part: u8, input: &str) -> String {
    match part {
        1 => part_one(&input),
        2 => part_two(&input),
        _ => panic!("There is no part {} for this day", part),
    }
}

#[derive(PartialEq, Debug)]
struct Move {
    shape: Shape,
    score: i32,
    beats: Shape,
    loses: Shape,
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
                loses: Shape::Rock,
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

fn part_one(input: &str) -> String {
    let mut accum_score = 0;
    for round in input.lines() {
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

    accum_score.to_string()
}

fn part_two(input: &str) -> String {
    let mut accum_score = 0;
    for round in input.lines() {
        let opponent_move = round.split(' ').nth(0).unwrap().parse::<Move>().unwrap();
        let outcome = match round.split(' ').nth(1).unwrap() {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            _ => Outcome::Win,
        };

        let my_move = match outcome {
            Outcome::Win => Move::new(opponent_move.loses),
            Outcome::Draw => Move::new(opponent_move.shape),
            Outcome::Lose => Move::new(opponent_move.beats),
        };

        let score = my_move.score + outcome as i32;
        accum_score += score;
    }

    accum_score.to_string()
}
