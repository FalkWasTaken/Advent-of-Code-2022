use std::collections::HashSet;
use utils::get_input;

fn unique(chars: &[char]) -> bool {
    chars.len() == chars.iter().collect::<HashSet<_>>().len()
}

fn unique_window(stream: &Vec<char>, window_size: usize) -> usize {
    stream
        .windows(window_size)
        .enumerate()
        .find(|(_, w)| unique(w))
        .unwrap()
        .0
        + window_size
}

fn solve1(stream: &Vec<char>) {
    let res = unique_window(stream, 4);
    println!("{res} characters need to be processed before the first start-of-packet marker.")
}

fn solve2(stream: &Vec<char>) {
    let res = unique_window(stream, 14);
    println!("{res} characters need to be processed before the first start-of-message marker.")
}

fn main() {
    let input = get_input(6);
    let stream = input.chars().collect();
    solve1(&stream);
    solve2(&stream);
}
