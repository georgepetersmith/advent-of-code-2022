use std::env;
use std::fs::File;
use std::io::{prelude::*, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut file = File::open(&file_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let calories = buffer
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calorie| calorie.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted_by(|a, b| b.cmp(a))
        .collect::<Vec<_>>();

    let max_calories = calories.iter().max().expect("Error");
    let top_three_calories_sum = calories.iter().take(3).sum::<u32>();

    println!("Part One: {}", max_calories);
    println!("Part Two: {}", top_three_calories_sum);

    Ok(())
}
