use std::collections::HashMap;
use std::io::Result;

fn main() -> Result<()> {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
";

    part_one(input)?;

    Ok(())
}

fn part_one(input: &str) -> Result<()> {
    println!("{}", input);

    let crate_indexes = [1, 5, 9];
    let mut rows = input
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

    let mut stacks: HashMap<usize, Vec<Option<char>>> =
        HashMap::from([(1, Vec::new()), (2, Vec::new()), (3, Vec::new())]);
    for row in rows {
        for c in row {
            let list = stacks.get_mut(&c.0).unwrap();
            list.push(c.1);
        }
    }

    // Reversing lists to use push/pop for moving crates.
    // The answer will now be the first crate in each list
    for i in 1..=stacks.len() {
        let list = stacks.get_mut(&i).unwrap();
        list.reverse();
    }

    dbg!(stacks);
    Ok(())
}
