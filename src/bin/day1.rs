use std::{io::{stdin, Read, Error}, cmp::Reverse};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let mut callories: Vec<usize> = input
        .split_terminator("\n\n")
        .map(|elf| 
            elf
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .sum()
        )
        .collect();
    callories.sort_by_key(|&c| Reverse(c));
    let res: usize = callories[0..3].iter().sum();
    println!("{res}");
    Ok(())
}