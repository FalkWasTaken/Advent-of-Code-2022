#![feature(array_windows)]

use std::{
    collections::VecDeque,
    fmt::Display,
    io::{self, Write as IoWrite},
    ops::{Index, IndexMut},
};

use clap::{Parser, ValueEnum};
use itertools::Itertools;
use termion::{
    color::{self, Rgb},
    cursor::{Goto, HideCursor},
    raw::IntoRawMode,
};
use utils::*;

use Particle::*;

const AIR_COLOR: Rgb = Rgb(50, 50, 50);
const ROCK_COLOR: Rgb = Rgb(216, 216, 216);
const SAND_COLOR: Rgb = Rgb(245, 213, 159);

const FPS: u64 = 30;

#[derive(Parser)]
struct Args {
    input: InputType,
    #[arg(long)]
    floor: bool,
    #[arg(long)]
    fps: Option<u64>,
}

#[derive(Clone, ValueEnum)]
enum InputType {
    Simple,
    Long,
}

const TEST: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

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

impl Display for Particle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Air => f.write_fmt(format_args!("{}.", color::Fg(AIR_COLOR))),
            Rock => f.write_fmt(format_args!("{}#", color::Fg(ROCK_COLOR))),
            Sand => f.write_fmt(format_args!("{}o", color::Fg(SAND_COLOR))),
        }
    }
}

struct Grid {
    grid: Vec<VecDeque<Particle>>,
    src: Pos,
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
            grid: grid.into_iter().map(|row| row.into()).collect(),
            src: (500 - min_x, 0),
            has_floor: false,
        }
    }

    fn len(&self) -> (usize, usize) {
        (self.grid[0].len(), self.grid.len())
    }

    fn can_spawn(&self) -> bool {
        matches!(self[self.src], Air)
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

    fn display(&self) {
        let mut stdout = HideCursor::from(io::stdout()).into_raw_mode().unwrap();
        for (y, row) in self.grid.iter().enumerate() {
            write!(stdout, "{}", Goto(1, y as u16 + 1)).unwrap();
            for (x, &sqr) in row.iter().enumerate() {
                if (x, y) == self.src && !matches!(sqr, Sand) {
                    write!(stdout, "+").unwrap();
                } else {
                    write!(stdout, "{sqr}").unwrap();
                }
            }
        }
    }

    fn sim_parallel(&mut self, fps: u64) {
        let src = self.src;
        let mut sand = VecDeque::from([src]);
        let mut should_spawn = true;
        println!("{}", termion::clear::All);
        self.display();
        self[src] = Sand;
        while !sand.is_empty() {
            let mut next_queue = VecDeque::with_capacity(sand.len());
            while let Some(mut pos) = sand.pop_front() {
                if pos.1 == self.len().1 - 1 {
                    if !self.has_floor {
                        should_spawn = false;
                    }
                    continue;
                } else if pos.0 == 0 {
                    if self.has_floor {
                        self.extend_left(1);
                        next_queue.iter_mut().for_each(|p: &mut Pos| p.0 += 1);
                        sand.iter_mut().for_each(|p| p.0 += 1);
                        pos.0 += 1;
                    } else {
                        should_spawn = false;
                        self[pos] = Air;
                        continue;
                    }
                } else if pos.0 == self.len().0 - 1 {
                    if self.has_floor {
                        self.extend_right(1);
                    } else {
                        should_spawn = false;
                        self[pos] = Air;
                        continue;
                    }
                }
                match next_pos(pos).iter().find(|&&p| matches!(self[p], Air)) {
                    Some(&new_pos) => {
                        self[pos] = Air;
                        self[new_pos] = Sand;
                        next_queue.push_back(new_pos);
                    }
                    None => self[pos] = Sand,
                }
            }
            sand = next_queue;
            if should_spawn && self.can_spawn() {
                sand.push_back(self.src);
            }
            std::thread::sleep(std::time::Duration::from_millis(1000 / fps));
            self.display();
        }
    }
}

impl Drop for Grid {
    fn drop(&mut self) {
        println!("{}{}", termion::style::Reset, Goto(1, self.len().1 as u16))
    }
}

fn main() {
    let args = Args::parse();
    let input = match args.input {
        InputType::Simple => TEST.to_string(),
        InputType::Long => get_input(14),
    };
    let mut grid = Grid::new(input.lines().map(parse_path).collect());
    if args.floor {
        grid.add_floor();
    }
    match args.fps {
        Some(fps) => grid.sim_parallel(fps),
        None => grid.sim_parallel(FPS),
    }
}
