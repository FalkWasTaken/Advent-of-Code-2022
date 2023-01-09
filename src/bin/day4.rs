use utils::*;

#[derive(PartialEq, Eq)]
struct Range {
    from: usize,
    to: usize,
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let (from, to) = s.split_once("-").unwrap();
        Range {
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
        }
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else if self.from <= other.from && self.to >= other.to {
            Some(std::cmp::Ordering::Greater)
        } else if self.from >= other.from && self.to <= other.to {
            Some(std::cmp::Ordering::Less)
        } else {
            None
        }
    }
}

fn overlaps((a, b): &(Range, Range)) -> bool {
    (a.from >= b.from && a.from <= b.to)
        || (a.to >= b.from && a.to <= b.to)
        || (b.from >= a.from && b.from <= a.to)
        || (b.to >= a.from && b.to <= a.to)
}

fn line_to_ranges(line: &str) -> (Range, Range) {
    line.split_once(",").unwrap().map(Range::from)
}

fn problem1(input: &String) {
    let res = input
        .lines()
        .map(line_to_ranges)
        .filter_map(|(r1, r2)| r1.partial_cmp(&r2))
        .count();
    println!("Solution to problem 1: {res}");
}

fn problem2(input: &String) {
    let res = input.lines().map(line_to_ranges).filter(overlaps).count();
    println!("Solution to problem 2: {res}");
}

fn main() {
    let input = input!();
    problem1(&input);
    problem2(&input);
}
