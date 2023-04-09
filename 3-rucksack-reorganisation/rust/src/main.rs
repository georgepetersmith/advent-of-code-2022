use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use substring::Substring;

fn main() {
    let mut priorities = HashMap::new();
    {
        let mut priority = 1;
        for c in 'a'..='z' {
            priorities.insert(c, priority);
            priority += 1;
        }
        for c in 'A'..='Z' {
            priorities.insert(c, priority);
            priority += 1;
        }
    }

    let file_path = env::args().nth(1).unwrap();
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut accum = 0;
    for line in reader.lines() {
        let contents = line.unwrap();
        let rucksack_size = contents.len();
        let compartment_size = rucksack_size / 2;

        let compartment1 = contents.substring(0, compartment_size);
        let compartment2 = contents.substring(compartment_size, rucksack_size);
        let shared_priority: i32 = compartment2
            .chars()
            .unique()
            .map(|s| {
                if compartment1.contains(s) {
                    return priorities[&s];
                } else {
                    return 0;
                };
            })
            .sum();

        accum += shared_priority;
    }

    println!("{}", accum);
}
