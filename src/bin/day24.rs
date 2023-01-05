use std::{collections::HashSet, ops::Index};

use itertools::Itertools;
use utils::*;
use Blizzard::*;

const UNITS: [Pos; 5] = [(0, 0), (1, 0), (0, 1), (-1, 0), (0, -1)];

const START_POS: Pos = (0, -1);

#[derive(Clone, Copy)]
enum Blizzard {
    Left,
    Right,
    Up,
    Down,
}

type Pos = (isize, isize);

struct Grid {
    grid: Vec<Vec<Vec<Blizzard>>>,
    time: usize,
}

impl Index<Pos> for Grid {
    type Output = Vec<Blizzard>;
    fn index(&self, index: Pos) -> &Self::Output {
        &self.grid[index.1 as usize][index.0 as usize]
    }
}

impl Grid {
    fn parse(s: String) -> Grid {
        let grid = s
            .lines()
            .map(|l| {
                l.chars()
                    .filter_map(|c| match c {
                        '.' => Some(vec![]),
                        '<' => Some(vec![Left]),
                        '>' => Some(vec![Right]),
                        '^' => Some(vec![Up]),
                        'v' => Some(vec![Down]),
                        _ => None,
                    })
                    .collect_vec()
            })
            .filter(|r| r.len() > 1)
            .collect();
        Grid { grid, time: 0 }
    }

    fn len(&self) -> Pos {
        (self.grid[0].len() as isize, self.grid.len() as isize)
    }

    fn step_time(&mut self) {
        let len = self.len().map(|l| l as usize);
        let prev = std::mem::replace(&mut self.grid, vec![vec![vec![]; len.0]; len.1]);
        for (y, row) in prev.into_iter().enumerate() {
            for (x, sqr) in row.into_iter().enumerate() {
                for bliz in sqr {
                    match bliz {
                        Left => self.grid[y][(x - 1).min(len.0 - 1)].push(Left),
                        Right => self.grid[y][(x + 1) % len.0].push(Right),
                        Up => self.grid[(y - 1).min(len.1 - 1)][x].push(Up),
                        Down => self.grid[(y + 1) % len.1][x].push(Down),
                    }
                }
            }
        }
        self.time += 1;
    }

    fn valid_pos(&self, p: &Pos) -> bool {
        (p.0 >= 0 && p.0 < self.len().0 && p.1 >= 0 && p.1 < self.len().1)
            || *p == START_POS
            || *p == self.goal()
    }

    fn get_next(&self, from: Pos) -> impl Iterator<Item = Pos> + '_ {
        UNITS
            .iter()
            .map(move |p| (from.0 + p.0, from.1 + p.1))
            .filter(|p| self.valid_pos(p))
    }

    fn bfs(&mut self, start: Pos, goal: Pos) -> usize {
        let mut next_positions = HashSet::from([start]);
        loop {
            let current = std::mem::take(&mut next_positions);
            self.step_time();
            for p in current {
                for next in self.get_next(p) {
                    if next == goal {
                        return self.time;
                    } else if next == START_POS || next == self.goal() {
                        next_positions.insert(next);
                    } else if !self[next].is_empty() {
                        continue;
                    } else {
                        next_positions.insert(next);
                    }
                }
            }
        }
    }

    fn goal(&self) -> Pos {
        (self.len().0, self.len().1 - 1)
    }
}

fn main() {
    let mut grid = Grid::parse(input!());
    let res = grid.bfs(START_POS, grid.goal());
    println!("Solution to problem 1: {res}");

    grid.bfs(grid.goal(), START_POS);
    let res = grid.bfs(START_POS, grid.goal());
    println!("Solution to problem 2: {res}");
}
