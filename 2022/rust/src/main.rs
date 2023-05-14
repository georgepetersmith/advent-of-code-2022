use std::env::args;

mod day_1_calorie_counting;
mod day_2_rock_paper_scissors;
mod day_3_rucksack_reorganisation;
mod day_4_camp_cleanup;
mod day_5_supply_stacks;
mod day_6_tuning_trouble;

fn main() {
    let day = args().nth(1).unwrap();
    let part = args().nth(2).unwrap().parse::<u8>().unwrap();

    let input_path = std::fs::read_dir("../inputs/")
        .unwrap()
        .find(|p| {
            p.as_ref()
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .to_owned()
                .split("-")
                .nth(0)
                .unwrap()
                .eq(&day)
        })
        .expect("Input file should be present")
        .unwrap()
        .path();

    let input_path_str = input_path.to_str().unwrap();

    let input = std::fs::read_to_string(input_path_str).unwrap();

    let ans = match day.parse::<u8>().unwrap() {
        1 => day_1_calorie_counting::run_part(part, &input),
        2 => day_2_rock_paper_scissors::run_part(part, &input),
        3 => day_3_rucksack_reorganisation::run_part(part, &input),
        4 => day_4_camp_cleanup::run_part(part, &input),
        5 => day_5_supply_stacks::run_part(part, &input),
        6 => day_6_tuning_trouble::run_part(part, &input),
        _ => panic!("There is no module setup for day {}", day),
    };

    println!("Answer to day {} part {} is: {}", day, part, ans);
}
