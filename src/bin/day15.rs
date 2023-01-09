use itertools::Itertools;

use utils::*;

const UPPER_BOUND: isize = 4_000_000;
const Y_TARGET: isize = 2_000_000;

type Pos = (isize, isize);

fn dist(a: Pos, b: Pos) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn valid_pos(pos: &Pos) -> bool {
    matches!(pos, (0..=UPPER_BOUND, 0..=UPPER_BOUND))
}

struct Sensor {
    pos: Pos,
    beacon: Pos,
    dist: isize,
}

impl Sensor {
    fn parse(s: &str) -> Sensor {
        let (fst, snd) = s.split_once(": ").unwrap();
        let parse_pos = |st: &str, prefix| {
            st.strip_prefix(prefix)
                .unwrap()
                .split_once(", ")
                .unwrap()
                .map(|t| t[2..].parse().unwrap())
        };
        let pos = parse_pos(fst, "Sensor at ");
        let beacon = parse_pos(snd, "closest beacon is at ");
        Sensor {
            pos,
            beacon,
            dist: dist(pos, beacon),
        }
    }

    fn in_reach(&self, pos: Pos) -> bool {
        dist(pos, self.pos) <= self.dist
    }

    fn border(&self) -> impl Iterator<Item = Pos> + '_ {
        let pos = self.pos;
        let x_range = (pos.0 - self.dist - 1).max(0)..(pos.0 + self.dist + 1).min(UPPER_BOUND);
        let y = move |x: isize, sign| pos.1 + sign * (self.dist + 1 - (pos.0 - x).abs());
        x_range
            .clone()
            .map(move |x| (x, y(x, -1)))
            .chain(x_range.map(move |x| (x, y(x, 1))))
    }
}

fn solve1(sensors: &Vec<Sensor>) {
    let (mut min_x, mut max_x) = sensors
        .iter()
        .flat_map(|s| [s.pos.0, s.beacon.0])
        .minmax()
        .into_option()
        .unwrap();
    while sensors.iter().any(|s| s.in_reach((min_x, Y_TARGET))) {
        min_x -= 10;
    }
    while sensors.iter().any(|s| s.in_reach((max_x, Y_TARGET))) {
        max_x += 10;
    }
    let res = (min_x..=max_x)
        .filter(|&x| {
            !sensors.iter().any(|s| s.beacon == (x, Y_TARGET))
                && sensors.iter().any(|s| s.in_reach((x, Y_TARGET)))
        })
        .count();
    println!("Solution to problem 1: {res}");
}

fn solve2(sensors: &Vec<Sensor>) {
    for s in sensors {
        for pos in s.border().filter(valid_pos) {
            if sensors.iter().all(|s| !s.in_reach(pos)) {
                println!("Solution to problem 2: {}", pos.0 * UPPER_BOUND + pos.1);
                return;
            }
        }
    }
}

fn main() {
    let sensors = input!().lines().map(Sensor::parse).collect();
    solve1(&sensors);
    solve2(&sensors);
}
