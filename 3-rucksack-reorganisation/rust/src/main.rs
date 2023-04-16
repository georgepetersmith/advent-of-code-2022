use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let input = read_to_string(file_path).unwrap();
    let lines = input.lines().collect();

    let priorities = ('a'..='z')
        .chain('A'..='Z')
        .into_iter()
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect::<HashMap<char, usize>>();

    part_one(&lines, &priorities);
    part_two(&lines, &priorities);
}

fn part_one(lines: &Vec<&str>, priorities: &HashMap<char, usize>) {
    let total_priority = lines
        .iter()
        .map(|s| {
            let (compartment1, compartment2) = s.split_at(s.len() / 2);
            let common_char = compartment1
                .chars()
                .find(|c| compartment2.contains(*c))
                .unwrap();

            priorities.get(&common_char).unwrap()
        })
        .sum::<usize>();

    println!("Part One: {}", total_priority);
}

fn part_two(lines: &Vec<&str>, priorities: &HashMap<char, usize>) {
    // This can be replaced with the array_chunks method in the nightly API.
    // https://doc.rust-lang.org/std/slice/struct.ArrayChunks.html
    let mut array_chunks: Vec<[&str; 3]> = Vec::new();
    let mut chunk: [&str; 3] = ["", "", ""];
    let mut slice_iter = 0;
    for line in lines {
        chunk[slice_iter] = line;
        slice_iter += 1;
        if slice_iter == 3 {
            array_chunks.push(chunk);
            chunk = ["", "", ""];
            slice_iter = 0;
        }
    }

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

    println!("Part Two: {}", total_priority);
}
