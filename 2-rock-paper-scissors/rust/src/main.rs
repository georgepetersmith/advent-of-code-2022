use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

fn main() -> Result<()> {
    let shape_scores: HashMap<char, i32> = HashMap::from([('A', 1), ('X', 1), ('B', 2), ('Y', 2), ('C', 3), ('Z', 3)]);
    let outcome_scores: HashMap<&str, i32> = HashMap::from([("LOST", 0), ("DREW", 3), ("WON", 6)]);

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut accum_score = 0;
    for line in reader.lines() {
        if let Ok(strategy) = line {
            let my_shape = strategy.chars().nth(2).unwrap();
            let shape_score = shape_scores.get(&my_shape).unwrap();

            let opponent_shape = strategy.chars().nth(0).unwrap();
            let outcome = match my_shape {
                'X' => {
                    match opponent_shape {
                        'A' => "DREW",
                        'B' => "LOST",
                        'C' => "WON",
                        _ => "ERROR"
                    }
                },
                'Y' => {
                    match opponent_shape {
                        'A' => "WON",
                        'B' => "DREW",
                        'C' => "LOST",
                        _ => "ERROR"
                    }
                },
                'Z' => {
                    match opponent_shape {
                        'A' => "LOST",
                        'B' => "WON",
                        'C' => "DREW",
                        _ => "ERROR"
                    }
                },
                _ => "ERROR"
            };

            let outcome_score = outcome_scores.get(&outcome).unwrap();

            let score = shape_score + outcome_score;
            accum_score += score;
        }
    }

    println!("Part One Answer: {}", accum_score);

    Ok(())
}
