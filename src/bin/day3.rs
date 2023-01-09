use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use utils::*;

lazy_static! {
    static ref PRIORITIES: HashMap<char, usize> = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect();
}

fn char_set(s: &str) -> HashSet<char> {
    s.chars().collect()
}

fn solve1(input: &String) {
    let res = input
        .lines()
        .map(|l| l.split_at(l.len() / 2).map(char_set))
        .map(|(a, b)| a.intersection(&b).map(|c| PRIORITIES[c]).sum::<usize>())
        .sum::<usize>();
    println!("Answer to problem 1: {res}");
}

fn solve2(input: &String) {
    let res = input
        .lines()
        .map(|s| char_set(s))
        .collect_vec()
        .chunks(3)
        .map(|g| (&(&g[0] & &g[1]) & &g[2]))
        .map(|b| PRIORITIES[b.iter().pop()])
        .sum::<usize>();
    println!("Answer to problem 2: {res}");
}

fn main() {
    let input = get_input(3);
    solve1(&input);
    solve2(&input);
}
