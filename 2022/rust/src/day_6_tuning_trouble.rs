use std::collections::HashSet;

pub fn run_part(part: u8, input: &str) -> String {
    match part {
        1 => part_one(input),
        _ => panic!("There is no part {} for this day", part),
    }
}

fn part_one(input: &str) -> String {
    let mut start_index = 0;
    let mut end_index = 4;
    let mut ans = 0;

    while end_index < input.len() {
        let slice = input
            .chars()
            .skip(start_index)
            .take(4)
            .collect::<HashSet<char>>();

        if slice.len() == 4 {
            ans = end_index;
            break;
        }

        start_index = start_index + 1;
        end_index = end_index + 1;
    }

    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let expect = "5".to_owned();
        let sut = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = part_one(sut);
        assert_eq!(result, expect);
    }
}
