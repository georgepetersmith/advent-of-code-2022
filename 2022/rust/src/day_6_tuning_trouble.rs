use std::collections::HashSet;

pub fn run_part(part: u8, input: &str) -> String {
    match part {
        1 => part_one(input).to_string(),
        2 => part_two(input).to_string(),
        _ => panic!("There is no part {} for this day", part),
    }
}

fn part_one(input: &str) -> usize {
    search_marker_index(input, 4)
}

fn part_two(input: &str) -> usize {
    search_marker_index(input, 14)
}

fn search_marker_index(input: &str, len: usize) -> usize {
    let mut start_index = 0;
    let mut end_index = len;

    while end_index < input.len() {
        let slice = input
            .chars()
            .skip(start_index)
            .take(len)
            .collect::<HashSet<char>>();

        if slice.len() == len {
            break;
        }

        start_index = start_index + 1;
        end_index = end_index + 1;
    }

    end_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let expect = 5;
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = part_one(input);
        assert_eq!(result, expect);
    }

    #[test]
    fn part_two_test() {
        let expect = 19;
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = part_two(input);
        assert_eq!(result, expect);
    }
}
