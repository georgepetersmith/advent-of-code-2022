use std::cell::RefCell;
use std::collections::HashMap;

fn main() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    part_one(input);
}

fn part_one(input: &str) {
    let inputs = input.split("\n\n").collect::<Vec<&str>>();

    let crate_indexes = [1, 5, 9];
    let mut rows = inputs[0]
        .lines()
        .map(|l| {
            l.chars()
                .enumerate()
                .filter(|(i, _)| crate_indexes.contains(i))
                .map(|t| if t.1 == ' ' { None } else { Some(t.1) })
                .enumerate()
                .map(|(i, c)| (i + 1, c))
                .collect::<HashMap<usize, Option<char>>>()
        })
        .collect::<Vec<HashMap<usize, Option<char>>>>();

    // Remove stack indexes
    rows.pop();

    let stacks: HashMap<usize, RefCell<Vec<char>>> = HashMap::from([
        (1, RefCell::new(Vec::new())),
        (2, RefCell::new(Vec::new())),
        (3, RefCell::new(Vec::new())),
    ]);

    for row in rows {
        for c in row {
            if let Some(x) = c.1 {
                let mut list = stacks.get(&c.0).unwrap().borrow_mut();
                list.push(x);
            }
        }
    }

    // Reversing lists to use push/pop for moving crates.
    // The answer will now be the last crate in each vec.
    for i in 1..=stacks.len() {
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

        for _ in 0..quantity {
            if let Some(crate_char) = from_stack.pop() {
                to_stack.push(crate_char);
            }
        }
    }

    for i in 1..=stacks.len() {
        let stack = stacks.get(&i).unwrap().borrow();
        let answer = stack.last();
        if let Some(x) = answer {
            println!("{}", x);
        }
    }
}
