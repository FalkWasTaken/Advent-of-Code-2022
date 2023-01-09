use std::{
    collections::{HashSet, VecDeque},
    ops::{Add, Index, IndexMut},
};

use utils::*;
use Particle::*;

const UNITS: [Pos; 6] = [
    Pos::new(1, 0, 0),
    Pos::new(0, 1, 0),
    Pos::new(0, 0, 1),
    Pos::new(-1, 0, 0),
    Pos::new(0, -1, 0),
    Pos::new(0, 0, -1),
];

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: isize,
    y: isize,
    z: isize,
}

impl Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, rhs: Pos) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Pos {
    const fn new(x: isize, y: isize, z: isize) -> Pos {
        Pos { x, y, z }
    }

    fn parse(s: &str) -> Pos {
        let (x, y, z) = s.split(',').flat_map(str::parse).pop3();
        Pos { x, y, z }
    }

    fn around(&self) -> impl Iterator<Item = Pos> + '_ {
        UNITS.iter().map(|&u| u + *self)
    }

    fn is_clamped(&self, max_x: isize, max_y: isize, max_z: isize) -> bool {
        self.x >= 0
            && self.y >= 0
            && self.z >= 0
            && self.x < max_x
            && self.y < max_y
            && self.z < max_z
    }
}

fn solve1(positions: &Vec<Pos>) {
    let set: HashSet<_> = positions.clone().into_iter().collect();
    let res = set
        .iter()
        .flat_map(Pos::around)
        .filter(|p| !set.contains(p))
        .count();
    println!("Solution to problem 1: {res}");
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Particle {
    Lava,
    Air,
    Steam,
}

struct Space(Vec<Vec<Vec<Particle>>>);

impl Index<Pos> for Space {
    type Output = Particle;
    fn index(&self, index: Pos) -> &Self::Output {
        &self.0[index.z as usize][index.y as usize][index.x as usize]
    }
}

impl IndexMut<Pos> for Space {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self.0[index.z as usize][index.y as usize][index.x as usize]
    }
}

fn solve2(positions: &Vec<Pos>) {
    let max_x = positions.iter().max_by_key(|p| p.x).unwrap().x + 5;
    let max_y = positions.iter().max_by_key(|p| p.y).unwrap().y + 5;
    let max_z = positions.iter().max_by_key(|p| p.z).unwrap().z + 5;
    let mut space = Space(vec![
        vec![vec![Air; max_x as usize]; max_y as usize];
        max_z as usize
    ]);
    for &p in positions {
        space[p + Pos::new(1, 1, 1)] = Lava;
    }
    let mut res = 0;
    let mut queue = VecDeque::from([Pos::new(0, 0, 0)]);
    while let Some(p) = queue.pop_front() {
        if space[p] == Steam {
            continue;
        } else if space[p] == Lava {
            res += 1;
            continue;
        }
        space[p] = Steam;
        queue.extend(p.around().filter(|p| p.is_clamped(max_x, max_y, max_z)));
    }
    println!("Solution to problem 2: {res}");
}

fn main() {
    let positions = input!().lines().map(Pos::parse).collect();
    solve1(&positions);
    solve2(&positions);
}
