use std::collections::HashMap;

pub fn run_part(part: u8, input: &str) -> String {
    match part {
        1 => part_one(&input),
        2 => part_two(&input),
        _ => panic!("There is no part {} for this day", part),
    }
}

fn get_priorities() -> HashMap<char, usize> {
    ('a'..='z')
        .chain('A'..='Z')
        .into_iter()
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect::<HashMap<char, usize>>()
}

fn part_one(input: &str) -> String {
    let priorities = get_priorities();
    let total_priority = input
        .lines()
        .map(|s| {
            let (compartment1, compartment2) = s.split_at(s.len() / 2);
            let common_char = compartment1
                .chars()
                .find(|c| compartment2.contains(*c))
                .unwrap();

            priorities.get(&common_char).unwrap()
        })
        .sum::<usize>();

    total_priority.to_string()
}

fn part_two(input: &str) -> String {
    // This can be replaced with the array_chunks method in the nightly API.
    // https://doc.rust-lang.org/std/slice/struct.ArrayChunks.html
    let mut array_chunks: Vec<[&str; 3]> = Vec::new();
    let mut chunk: [&str; 3] = ["", "", ""];
    let mut slice_iter = 0;
    for line in input.lines() {
        chunk[slice_iter] = line;
        slice_iter += 1;
        if slice_iter == 3 {
            array_chunks.push(chunk);
            chunk = ["", "", ""];
            slice_iter = 0;
        }
    }

    let priorities = get_priorities();
    let total_priority = array_chunks
        .iter()
        .map(|e| {
            let elf1 = e.get(0).unwrap();
            let common_char = elf1
                .chars()
                .find(|c| {
                    let elf2 = e.get(1).unwrap();
                    let elf3 = e.get(2).unwrap();
                    elf2.contains(*c) && elf3.contains(*c)
                })
                .unwrap();

            priorities.get(&common_char).unwrap()
        })
        .sum::<usize>();

    total_priority.to_string()
}
