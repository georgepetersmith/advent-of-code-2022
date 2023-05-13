use itertools::Itertools;

pub fn run_part(part: u8, input: &str) -> String {
    match part {
        1 => part_one(&input),
        2 => part_two(&input),
        _ => panic!("There is no part {} for this day", part),
    }
}

fn get_calories(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calorie| calorie.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted_by(|a, b| b.cmp(a))
        .collect::<Vec<_>>()
}

fn part_one(input: &str) -> String {
    let calories = get_calories(input);
    let max_calories = calories.iter().max().expect("Error");
    max_calories.to_string()
}

fn part_two(input: &str) -> String {
    let calories = get_calories(input);
    let top_three_calories_sum = calories.iter().take(3).sum::<u32>();
    top_three_calories_sum.to_string()
}
