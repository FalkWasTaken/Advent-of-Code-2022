use itertools::Itertools;
use std::cmp::Reverse;
use utils::input;

fn main() {
    let res: u32 = input!()
        .split("\n\n")
        .map(|elf| elf.lines().flat_map(str::parse::<u32>).sum::<u32>())
        .sorted_by_key(|&c| Reverse(c))
        .take(3)
        .sum();
    println!("{res}");
}
