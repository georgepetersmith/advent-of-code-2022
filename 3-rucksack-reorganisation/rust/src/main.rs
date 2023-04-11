use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use substring::Substring;


fn main() {
    let file_path = env::args().nth(1).unwrap();
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|s| s.unwrap()).collect::<Vec<String>>();

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

    part_one(&lines, &priorities);
    part_two(&lines, &priorities);
}

fn part_one(lines: &Vec<String>, priorities: &HashMap<char, i32>) {
    let mut accum = 0;
    for contents in lines.iter() {
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

    println!("Part One: {}", accum);
}

fn part_two(lines: &Vec<String>, priorities: &HashMap<char, i32>) {
    let mut groups: Vec<[&str; 3]> = Vec::new();
    let mut group: [&str; 3] = ["", "", ""];
    let mut slice_iter = 0;
    for contents in lines.iter() {
        group[slice_iter] = contents.as_str();
        slice_iter += 1;
        if slice_iter == 3 {
            groups.push(group.clone());
            group = ["", "", ""];
            slice_iter = 0;
        }
    }

    let mut accum = 0;
    let mut group_badge = None;
    for g in groups {
        for c in g[0].chars() {
            if g[1].contains(c) && g[2].contains(c) {
                group_badge = Some(c);
                break;
            }
        }

        let priority = priorities[&group_badge.unwrap()];
        accum += priority;
    }

    println!("Part Two: {}", accum);
}
