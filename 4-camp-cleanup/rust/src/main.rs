use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut part_one_count = 0;
    let mut part_two_count = 0;
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let sections = line.split(',').collect::<Vec<&str>>();

        let mut first_section_lower = 0;
        let mut first_section_upper = 0;
        let mut last_section_lower = 0;
        let mut last_section_upper = 0;

        let mut is_first = true;
        for section in sections.iter() {
            let bounds = section
                .split('-')
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            if is_first {
                first_section_lower = bounds.first().unwrap().to_owned();
                first_section_upper = bounds.last().unwrap().to_owned();

                is_first = false;
            } else {
                last_section_lower = bounds.first().unwrap().to_owned();
                last_section_upper = bounds.last().unwrap().to_owned();
            }
        }

        if (first_section_lower <= last_section_lower && first_section_upper >= last_section_upper)
            || (last_section_lower <= first_section_lower
                && last_section_upper >= first_section_upper)
        {
            part_one_count += 1;
            part_two_count += 1;
        } else if first_section_lower <= last_section_lower
            && last_section_lower <= first_section_upper
        {
            part_two_count += 1;
        } else if last_section_lower <= first_section_lower
            && first_section_lower <= last_section_upper
        {
            part_two_count += 1;
        }
    }

    println!("Part One: {}", part_one_count);
    println!("Part One: {}", part_two_count);
}
