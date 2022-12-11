use std::cmp::Reverse;
use utils::{get_input, ExtendedIter};

fn main() {
    let mut callories = get_input(1)
        .split_terminator("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<usize>().unwrap()).sum())
        .collect_vec();
    callories.sort_by_key(|&c| Reverse(c));
    let res: usize = callories[0..3].iter().sum();
    println!("{res}");
}
