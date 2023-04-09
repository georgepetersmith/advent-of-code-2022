use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut calorie_count = 0;
    let mut highest_calorie_count = 0;

    for line in reader.lines() {
        if let Ok(calories) = line {
            if calories.is_empty() {
                if calorie_count > highest_calorie_count {
                    highest_calorie_count = calorie_count;
                }

                calorie_count = 0;

                continue;
            }

            calorie_count += calories.parse::<i32>().unwrap();
        }
    }
}
