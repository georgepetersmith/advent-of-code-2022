use std::env;
use std::fs::File;
use std::io::{prelude::*, Result};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut file = File::open(file_path)?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let calorie_count = buffer
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calorie| calorie.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();

    println!("{}", calorie_count);

    Ok(())
}
