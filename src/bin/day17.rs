use std::{iter::Cycle, ops::Range, slice::Iter};

use itertools::Itertools;
use utils::*;

use Direction::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '<' => Left,
            '>' => Right,
            _ => panic!("Invalid input!"),
        }
    }
}

type Pos = (usize, usize);
type Shape = Vec<Pos>;

const MAX_NUM: usize = 1_000_000_000_000;
const GRID_SIZE: usize = 1_000;

struct Simulator<'a> {
    input: Cycle<Iter<'a, Direction>>,
    generator: Cycle<Range<u8>>,
    grid: [[bool; 7]; GRID_SIZE],
    highest_points: [usize; 7],
    base: usize,
    num_resizes: usize,
}

impl Simulator<'_> {
    fn new(input: &Vec<Direction>) -> Simulator {
        Simulator {
            input: input.iter().cycle(),
            generator: (0..5).cycle(),
            grid: [[false; 7]; GRID_SIZE].into(),
            highest_points: [0; 7],
            base: 0,
            num_resizes: 0,
        }
    }

    fn highest_point(&self) -> usize {
        *self.highest_points.iter().max().unwrap()
    }

    fn highest_point_absolute(&self) -> usize {
        self.highest_point() + self.base
    }

    fn resize(&mut self) {
        self.base += GRID_SIZE / 2;
        self.grid.rotate_left(self.base);
        for row in &mut self.grid[self.base..] {
            *row = [false; 7];
        }
        for y in &mut self.highest_points {
            assert!(*y >= self.base);
            *y -= self.base
        }
        self.num_resizes += 1;
    }

    fn initial_y(&mut self) -> usize {
        let mut y = self.highest_point() + 4;
        if y >= self.grid.len() - 10 {
            self.resize();
            y -= GRID_SIZE / 2;
        }
        y
    }

    fn spawn(&mut self) -> Shape {
        let y = self.initial_y();
        let shape = match self.generator.pop() {
            0 => (2..=5).map(|x| (x, y)).collect(),
            1 => {
                vec![(3, y), (3, y + 1), (3, y + 2), (2, y + 1), (4, y + 1)]
            }
            2 => {
                vec![(2, y), (3, y), (4, y), (4, y + 1), (4, y + 2)]
            }
            3 => (y..y + 4).map(|y| (2, y)).collect(),
            _ => {
                vec![(2, y), (3, y), (2, y + 1), (3, y + 1)]
            }
        };
        shape
    }

    fn fall(&mut self, shape: &mut Shape) -> bool {
        if shape.iter().any(|p| p.1 == 1 || self.grid[p.1 - 1][p.0]) {
            false
        } else {
            shape.iter_mut().for_each(|p| p.1 -= 1);
            true
        }
    }

    fn push(&mut self, shape: &mut Shape, dir: Direction) {
        match dir {
            Left => {
                if !shape.iter().any(|p| p.0 == 0 || self.grid[p.1][p.0 - 1]) {
                    shape.iter_mut().for_each(|p| p.0 -= 1);
                }
            }
            Right => {
                if !shape.iter().any(|p| p.0 == 6 || self.grid[p.1][p.0 + 1]) {
                    shape.iter_mut().for_each(|p| p.0 += 1);
                }
            }
        }
    }

    fn sim1(&mut self) {
        let mut shape = self.spawn();
        while let Some(&dir) = self.input.next() {
            self.push(&mut shape, dir);
            if !self.fall(&mut shape) {
                break;
            }
        }
        for (x, y) in shape {
            self.grid[y][x] = true;
            self.highest_points[x] = self.highest_points[x].max(y);
        }
    }

    fn sim(&mut self, n: usize) -> usize {
        for _ in 0..n {
            self.sim1();
        }
        self.highest_point_absolute()
    }
}

const CHECK_LIM: usize = 5;

fn solve1(input: &Vec<Direction>) {
    let mut s1 = Simulator::new(&input);
    let res = s1.sim(2022);
    println!("Solution to problem 1: {res}");
}

fn solve2(input: &Vec<Direction>) {
    let cycle = match input.len() % 5 {
        0 => input.len(),
        _ => input.len() * 5
    };
    let mut cache = vec![0];
    let mut s = Simulator::new(input);
    for k in 1.. {
        let res = s.sim(cycle);
        cache.push(res);
        if k % CHECK_LIM == 0
            && (0..CHECK_LIM)
                .map(|i| cache[(i + 1) * k / CHECK_LIM] - cache[i * k / CHECK_LIM])
                .all_equal()
        {
            let cycle = cycle * k / CHECK_LIM;
            let cycle_height = cache[k / CHECK_LIM];
            let num_cycles = MAX_NUM / cycle;
            let mut s = Simulator::new(input);
            let remainder_height = s.sim(MAX_NUM % cycle);
            println!("Solution to problem 2: {}", cycle_height * num_cycles + remainder_height);
            return;
        }
    }
    unreachable!()
}

fn main() {
    let input = input!().chars().map(Direction::from).collect();
    solve1(&input);
    solve2(&input);
}
