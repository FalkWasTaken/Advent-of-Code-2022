use itertools::Itertools;

fn unique_window(stream: &Vec<char>, window_size: usize) -> usize {
    stream
        .windows(window_size)
        .enumerate()
        .find(|(_, w)| w.iter().all_unique())
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
    let input = std::fs::read_to_string("inputs/day6.in").unwrap();
    let stream = input.chars().collect();
    solve1(&stream);
    solve2(&stream);
}
