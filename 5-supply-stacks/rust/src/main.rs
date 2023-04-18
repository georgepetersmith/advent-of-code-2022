use std::collections::HashMap;
use std::io::Result;

fn main() -> Result<()> {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    part_one(input)?;

    Ok(())
}

fn part_one(input: &str) -> Result<()> {
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

    let mut stacks: HashMap<usize, Vec<char>> =
        HashMap::from([(1, Vec::new()), (2, Vec::new()), (3, Vec::new())]);

    for row in rows {
        for c in row {
            if let Some(x) = c.1 {
                let list = stacks.get_mut(&c.0).unwrap();
                list.push(x);
            }
        }
    }

    // Reversing lists to use push/pop for moving crates.
    // The answer will now be the first crate in each list
    for i in 1..=stacks.len() {
        let list = stacks.get_mut(&i).unwrap();
        list.reverse();
    }

    let actions = inputs[1].lines();
    for action in actions {
        let _ = action
            .replace("move ", "")
            .replace("from ", "")
            .replace("to ", "")
            .split(' ')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
    }

    dbg!(stacks);

    Ok(())
}
