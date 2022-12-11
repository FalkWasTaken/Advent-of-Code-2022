use utils::{get_input, TupleMap};

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

impl Range {
    fn overlaps_with(&self, other: &Range) -> bool {
        (self.from >= other.from && self.from <= other.to)
            || (self.to >= other.from && self.to <= other.to)
            || (other.from >= self.from && other.from <= self.to)
            || (other.to >= self.from && other.to <= self.to)
    }
}

fn line_to_ranges(line: &str) -> (Range, Range) {
    line.split_once(",").unwrap().map(Range::from)
}

fn problem1(input: &String) {
    let res = input
        .lines()
        .map(|l| line_to_ranges(l))
        .filter_map(|(r1, r2)| r1.partial_cmp(&r2))
        .count();
    println!("Solution to problem 1: {res}");
}

fn problem2(input: &String) {
    let res = input
        .lines()
        .map(|l| line_to_ranges(l))
        .filter(|(r1, r2)| r1.overlaps_with(&r2))
        .count();
    println!("Solution to problem 1: {res}");
}

fn main() {
    let input = get_input(4);
    problem1(&input);
    problem2(&input);
}
