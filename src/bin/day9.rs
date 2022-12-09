use itertools::Itertools;
use std::{collections::HashSet, ops::Add};

const TEST: &str = 
"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

type Pos = (isize, isize);

struct Move(isize, isize);

impl From<&str> for Move {
    fn from(line: &str) -> Self {
        let (dir, steps) = line.split_once(" ").unwrap();
        let steps = steps.parse().unwrap();
        match dir {
            "R" => Move(0, steps),
            "L" => Move(0, -steps),
            "U" => Move(steps, 0),
            "D" => Move(-steps, 0),
            _ => panic!("Invalid direction!")
        }
    }
}

struct Rope {
    head: Pos,
    tail: Pos,
    visited: HashSet<Pos>
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: (0, 0),
            tail: (0, 0),
            visited: [(0, 0)].into()
        }
    }

    fn x_diff(&self) -> isize {
        (self.head.0 - self.tail.0).abs()
    }

    fn y_diff(&self) -> isize {
        (self.head.1 - self.tail.1).abs()
    }

    fn move_x(&mut self, mv: isize) {
        let dir = mv.signum();
        for _ in 0..mv.abs() {
            self.head.0 += dir;
            if self.x_diff() > 1 {
                self.tail.0 += dir;
                self.tail.1 = self.head.1;
                self.visited.insert(self.tail);
            }
        }
    }

    fn move_y(&mut self, mv: isize) {
        let dir = mv.signum();
        for _ in 0..mv.abs() {
            self.head.1 += dir;
            if self.y_diff() > 1 {
                self.tail.1 += dir;
                self.tail.0 = self.head.0;
                self.visited.insert(self.tail);
            }
        }
    }

    fn perform_move(&mut self, mv: Move) {
        match mv {
            Move(x, 0) => self.move_x(x),
            Move(0, y) => self.move_y(y),
            _ => panic!("Invalid move!")
        }
    }
}

fn main() {
    //let input = TEST;
    let input = std::fs::read_to_string("inputs/day9.in").unwrap();
    let moves = input.lines().map(Move::from).collect_vec();
    let mut rope = Rope::new();
    for mv in moves {
        rope.perform_move(mv);
    }
    //println!("{:?}", rope.visited);
    println!("{}", rope.visited.len())
}