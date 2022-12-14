#![feature(array_windows)]

use std::{
    collections::VecDeque,
    fmt::{Debug, Display, Write as FmtWrite},
    ops::{Index, IndexMut},
    io::Write as IoWrite
};

use itertools::Itertools;
use utils::*;

use Particle::*;

type Pos = (usize, usize);

fn parse_pos(s: &str) -> Pos {
    s.split_once(",").unwrap().map(|n| n.parse().unwrap())
}

fn next_pos(p: Pos) -> [Pos; 3] {
    [(p.0, p.1 + 1), (p.0 - 1, p.1 + 1), (p.0 + 1, p.1 + 1)]
}

type Path = Vec<Pos>;

fn parse_path(s: &str) -> Path {
    s.split(" -> ").map(parse_pos).collect()
}

#[derive(Clone, Copy)]
enum Particle {
    Air,
    Rock,
    Sand,
}

struct Grid {
    grid: Vec<VecDeque<Particle>>,
    src: Pos,
    counter: usize,
    has_floor: bool,
}

impl Index<Pos> for Grid {
    type Output = Particle;
    fn index(&self, index: Pos) -> &Self::Output {
        &self.grid[index.1][index.0]
    }
}

impl IndexMut<Pos> for Grid {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self.grid[index.1][index.0]
    }
}

impl Grid {
    fn new(paths: Vec<Path>) -> Grid {
        let flat_map = |f: fn(&Pos) -> usize| paths.iter().flatten().map(f);
        let (min_x, max_x) = flat_map(|t| t.0).minmax().into_option().unwrap();
        let max_y = flat_map(|t| t.1).max().unwrap();
        let max_x = max_x - min_x;
        let mut grid = vec![vec![Air; max_x + 1]; max_y + 1];
        for path in paths {
            for &[from, to] in path.array_windows() {
                for x in from.0.min(to.0)..=from.0.max(to.0) {
                    grid[from.1][x - min_x] = Rock;
                }
                for y in from.1.min(to.1)..=from.1.max(to.1) {
                    grid[y][from.0 - min_x] = Rock;
                }
            }
        }
        Grid {
            grid: grid.into_iter().map(VecDeque::from).collect(),
            src: (500 - min_x, 0),
            counter: 0,
            has_floor: false,
        }
    }

    fn len(&self) -> (usize, usize) {
        (self.grid[0].len(), self.grid.len())
    }

    fn can_continue(&self, from: Pos) -> bool {
        from.1 < self.len().1 - 1 && from.0 > 0 && from.0 < self.len().0 - 1
    }

    fn drop_sand(&mut self) -> bool {
        let mut pos = self.src;
        while self.can_continue(pos) {
            match next_pos(pos).iter().find(|&&p| matches!(self[p], Air)) {
                Some(&new_pos) => pos = new_pos,
                None => {
                    self[pos] = Sand;
                    return true;
                }
            }
        }
        false
    }

    fn drop_with_floor(&mut self) -> bool {
        let mut pos = self.src;
        if !matches!(self[pos], Air) {
            return false;
        }
        while pos.1 < self.len().1 - 1 {
            if pos.0 == 0 {
                self.extend_left(10);
                pos.0 += 10;
            } else if pos.0 == self.len().0 - 1 {
                self.extend_right(10);
            }
            match next_pos(pos).iter().find(|&&p| matches!(self[p], Air)) {
                Some(&new_pos) => pos = new_pos,
                None => {
                    self[pos] = Sand;
                    return true;
                }
            }
        }
        self[pos] = Sand;
        true
    }

    fn simulate(&mut self) {
        let drop = match self.has_floor {
            false => Grid::drop_sand,
            true => Grid::drop_with_floor,
        };    
        while drop(self) {
            self.counter += 1;
        }
    }

    fn clear(&mut self) {
        self.counter = 0;
        for row in &mut self.grid {
            for sqr in row.iter_mut().filter(|sqr| matches!(sqr, Sand)) {
                *sqr = Air;
            }
        }
    }

    fn add_floor(&mut self) {
        self.grid.push(vec![Air; self.len().0].into());
        self.has_floor = true;
    }

    fn extend_left(&mut self, n: usize) {
        self.src.0 += n;
        for row in &mut self.grid {
            for _ in 0..n {
                row.push_front(Air);
            }
        }
    }

    fn extend_right(&mut self, n: usize) {
        for row in &mut self.grid {
            row.append(&mut vec![Air; n].into());
        }
    }
}

fn main() {
    let mut grid = Grid::new(input!().lines().map(parse_path).collect());
    grid.simulate();
    println!("Solution to problem 1: {}", grid.counter);
    grid.clear();
    grid.add_floor();
    grid.simulate();
    println!("Solution to problem 2: {}", grid.counter);
}
