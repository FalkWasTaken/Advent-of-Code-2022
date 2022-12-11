use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use utils::{get_input, ExtendedIter};

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
    let mut res = 0;
    for line in input.lines() {
        let (a, b) = line.split_at(line.len() / 2);
        let (a, b) = (char_set(a), char_set(b));
        res += a
            .intersection(&b)
            .map(|c| PRIORITIES.get(c).unwrap())
            .sum::<usize>();
    }
    println!("Anser to problem 1: {res}");
}

fn solve2(input: &String) {
    let res = input
        .lines()
        .map(|s| char_set(s))
        .collect::<Vec<_>>()
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
