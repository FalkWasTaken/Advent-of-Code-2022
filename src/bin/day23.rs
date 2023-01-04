use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use itertools::Itertools;
use utils::*;

const NUM_ROUNDS: usize = 10;

const N: Pos = Pos(0, -1);
const S: Pos = Pos(0, 1);
const W: Pos = Pos(-1, 0);
const E: Pos = Pos(1, 0);
const NW: Pos = Pos(-1, -1);
const NE: Pos = Pos(1, -1);
const SW: Pos = Pos(-1, 1);
const SE: Pos = Pos(1, 1);

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Pos(isize, isize);

impl Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, rhs: Pos) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Pos {
    fn around(&self) -> impl Iterator<Item = Pos> + '_ {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|&p| p != (0, 0))
            .map(move |(x, y)| *self + Pos(x, y))
    }
}

fn perform_move(directions: &[[Pos; 3]; 4], positions: &mut HashSet<Pos>) -> bool {
    let mut moves: HashMap<_, _> = positions.iter().map(|&p| (p, p)).collect();
    for &p in positions.iter() {
        if !p.around().any(|p2| positions.contains(&p2)) {
            continue;
        }
        let check = |dirs: &[Pos]| dirs.iter().any(|&d| positions.contains(&(p + d)));
        for dirs in directions {
            if !check(dirs) {
                moves.insert(p, p + dirs[0]);
                break;
            }
        }
    }
    let mut found_new = false;
    let valid_next: HashSet<_> = moves
        .values()
        .counts()
        .into_iter()
        .filter(|&(_, c)| c == 1)
        .map(|t| *t.0)
        .collect();
    *positions = moves
        .into_iter()
        .map(|(from, to)| {
            if valid_next.contains(&to) {
                found_new = found_new || from != to;
                to
            } else {
                from
            }
        })
        .collect();
    found_new
} 

fn main() {
    let input = input!();
    let mut positions: HashSet<_> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| Pos(x as isize, y as isize))
        })
        .collect();
    let mut directions = [[N, NE, NW], [S, SE, SW], [W, NW, SW], [E, NE, SE]];
    for round in 1.. {
        if !perform_move(&directions, &mut positions) {
            println!("Solution to problem 2: {}", round);
            return;
        } else if round == NUM_ROUNDS {
            let minmax =
                |f: fn(&Pos) -> isize| positions.iter().map(f).minmax().into_option().unwrap();
            let (min_x, max_x) = minmax(|p| p.0);
            let (min_y, max_y) = minmax(|p| p.1);
            let res = (max_x - min_x + 1) * (max_y - min_y + 1) - positions.len() as isize;
            println!("Solution to problem 1: {res}");
        }
        directions.rotate_left(1);
    }
}
