use std::io::stdin;

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

fn line_to_ranges(line: String) -> (Range, Range) {
    let (r1, r2) = line.split_once(",").unwrap();
    (r1.into(), r2.into())
}

fn problem1() -> usize {
    stdin()
        .lines()
        .map(|l| line_to_ranges(l.unwrap()))
        .filter_map(|(r1, r2)| r1.partial_cmp(&r2))
        .count()
}

fn problem2() -> usize {
    stdin()
        .lines()
        .map(|l| line_to_ranges(l.unwrap()))
        .filter(|(r1, r2)| r1.overlaps_with(&r2))
        .count()
}

fn main() {
    //let res = problem1();
    let res = problem2();
    println!("{res}")
}
