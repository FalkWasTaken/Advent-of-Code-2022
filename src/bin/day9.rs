use itertools::Itertools;
use std::{
    collections::HashSet,
    ops::{Add, AddAssign, Sub},
};
use utils::get_input;

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos(isize, isize);

impl From<&str> for Pos {
    fn from(line: &str) -> Self {
        let (dir, steps) = line.split_once(" ").unwrap();
        let steps = steps.parse().unwrap();
        match dir {
            "R" => Pos(steps, 0),
            "L" => Pos(-steps, 0),
            "U" => Pos(0, steps),
            "D" => Pos(0, -steps),
            _ => panic!("Invalid direction!"),
        }
    }
}

impl Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, rhs: Pos) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<Pos> for Pos {
    fn add_assign(&mut self, rhs: Pos) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub<Pos> for Pos {
    type Output = Pos;
    fn sub(self, rhs: Pos) -> Self::Output {
        Pos(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Pos {
    fn abs(&self) -> isize {
        self.0.abs() + self.1.abs()
    }

    fn unit(&self) -> Pos {
        Pos(self.0.signum(), self.1.signum())
    }
}

struct Rope {
    knots: Vec<Pos>,
    visited: HashSet<Pos>,
}

impl Rope {
    fn new(n: usize) -> Rope {
        Rope {
            knots: vec![Pos(0, 0); n],
            visited: [Pos(0, 0)].into(),
        }
    }

    fn perform_move(&mut self, mv: Pos) {
        let knots = &mut self.knots;
        for _ in 0..mv.abs() {
            knots[0] += mv.unit();
            for i in 1..knots.len() {
                let diff = knots[i - 1] - knots[i];
                if diff.0.abs() * diff.1.abs() > 1 {
                    knots[i] += diff.unit();
                } else if diff.0.abs() > 1 {
                    knots[i].0 += diff.0.signum();
                } else if diff.1.abs() > 1 {
                    knots[i].1 += diff.1.signum();
                }
            }
            self.visited.insert(knots[knots.len() - 1]);
        }
    }
}

fn solve1(moves: &Vec<Pos>) {
    let mut rope = Rope::new(2);
    for &mv in moves {
        rope.perform_move(mv);
    }
    println!("Solution for problem 1: {}", rope.visited.len());
}

fn solve2(moves: &Vec<Pos>) {
    let mut rope = Rope::new(10);
    for &mv in moves {
        rope.perform_move(mv);
    }
    println!("Solution for problem 2: {}", rope.visited.len());
}

fn main() {
    let input = get_input(9);
    let moves = input.lines().map(Pos::from).collect_vec();
    solve1(&moves);
    solve2(&moves);
}
