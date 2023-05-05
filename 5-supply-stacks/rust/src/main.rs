use itertools::Itertools;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut buffer = String::new();
    let mut file = File::open(args().nth(1).unwrap()).unwrap();
    file.read_to_string(&mut buffer).unwrap();

    let part_one_answer = calculate_top_crates(&buffer, &mut move_crates_individually);
    println!("Part One Answer: {}", part_one_answer);

    let part_two_answer = calculate_top_crates(&buffer, &mut move_crates_grouped);
    println!("Part Two Answer: {}", part_two_answer);
}

fn move_crates_individually(
    from_stack: &mut RefMut<Vec<char>>,
    to_stack: &mut RefMut<Vec<char>>,
    quantity: usize,
) {
    for _ in 0..quantity {
        if let Some(crate_char) = from_stack.pop() {
            to_stack.push(crate_char);
        }
    }
}

fn move_crates_grouped(
    from_stack: &mut RefMut<Vec<char>>,
    to_stack: &mut RefMut<Vec<char>>,
    quantity: usize,
) {
    let split_index = from_stack.len() - quantity;
    let crates_to_move = from_stack.split_off(split_index);
    for crate_char in crates_to_move {
        to_stack.push(crate_char);
    }
}

fn calculate_top_crates(
    input: &str,
    move_crates: &mut dyn FnMut(&mut RefMut<Vec<char>>, &mut RefMut<Vec<char>>, usize),
) -> String {
    let inputs = input.split("\n\n").collect::<Vec<&str>>();

    let stack_quantity = (input.lines().next().unwrap().len() + 1) / 4;
    let crate_indexes = (1..=stack_quantity)
        .map(|i| (i * 4) - 3)
        .collect::<Vec<usize>>();

    let mut rows = inputs[0]
        .lines()
        .map(|l| {
            l.chars()
                .enumerate()
                .filter(|(i, _)| crate_indexes.contains(i))
                .map(|t| if t.1 == ' ' { None } else { Some(t.1) })
                .enumerate()
                .collect()
        })
        .collect::<Vec<Vec<(usize, Option<char>)>>>();

    // Remove stack indexes
    rows.pop();

    let mut stacks: HashMap<usize, RefCell<Vec<char>>> = HashMap::new();
    for row in rows {
        for c in row {
            if let Some(x) = c.1 {
                let list = stacks.entry(c.0 + 1).or_insert(RefCell::new(Vec::new()));
                list.borrow_mut().push(x);
            }
        }
    }

    // Reversing lists to use push/pop for moving crates.
    // The answer will now be the last crate in each vec.
    for i in 1..=stack_quantity {
        let mut list = stacks.get(&i).unwrap().borrow_mut();
        list.reverse();
    }

    let commands = inputs[1].lines();
    for command in commands {
        let sub_commands = command
            .replace("move ", "")
            .replace("from ", "")
            .replace("to ", "")
            .split(' ')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let quantity = sub_commands[0];
        let from_stack_index = sub_commands[1];
        let to_stack_index = sub_commands[2];

        let mut from_stack = stacks.get(&from_stack_index).unwrap().borrow_mut();
        let mut to_stack = stacks.get(&to_stack_index).unwrap().borrow_mut();

        move_crates(&mut from_stack, &mut to_stack, quantity)
    }

    let answer = stacks
        .iter()
        .sorted_by_key(|x| x.0)
        .fold(String::new(), |mut ans, c| {
            let cr = c.1.borrow().last().unwrap().clone();
            ans.push(cr);
            ans
        });

    answer
}
