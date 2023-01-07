use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use nom::{branch::alt, bytes::complete::tag, character::complete::digit1, multi::many1, IResult};
use utils::*;

use Direction::*;
use Instruction::*;
use Square::*;

type NextDir = (usize, Direction);

fn next_arr(l: NextDir, r: NextDir, u: NextDir, d: NextDir) -> HashMap<Direction, NextDir> {
    [(Left, l), (Right, r), (Up, u), (Down, d)].into()
}

lazy_static! {
    static ref NEXT: HashMap<usize, HashMap<Direction, (usize, Direction)>> = [
        (0, next_arr((3, Right), (1, Right), (5, Right), (2, Down))),
        (1, next_arr((0, Left), (4, Left), (5, Up), (2, Left))),
        (2, next_arr((3, Down), (1, Up), (0, Up), (4, Down))),
        (3, next_arr((0, Right), (4, Right), (2, Right), (5, Down))),
        (4, next_arr((3, Left), (1, Left), (2, Up), (5, Left))),
        (5, next_arr((0, Down), (4, Up), (3, Up), (1, Down)))
    ]
    .into();
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left = 2,
    Right = 0,
    Up = 3,
    Down = 1,
}

#[derive(Clone, Copy)]
enum Instruction {
    Turn(Direction),
    Walk(usize),
}

impl Instruction {
    fn parse(s: &str) -> IResult<&str, Instruction> {
        let (s, d) = alt((alt((tag("L"), tag("R"))), digit1))(s)?;
        match d {
            "L" => Ok((s, Turn(Left))),
            "R" => Ok((s, Turn(Right))),
            d => Ok((s, Walk(d.parse().unwrap()))),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Square {
    Empty,
    Wall,
    Null,
}

enum WalkRes {
    Ok,
    Wall,
    Wrap,
}

struct SideWalker {
    grid: [[Square; 50]; 50],
    facing: Direction,
    pos: (usize, usize),
    offset: (usize, usize),
}

impl SideWalker {
    fn turn(&mut self, dir: Direction) {
        match self.facing {
            Left => match dir {
                Left => self.facing = Down,
                _ => self.facing = Up,
            },
            Right => match dir {
                Left => self.facing = Up,
                _ => self.facing = Down,
            },
            Up => match dir {
                Left => self.facing = Left,
                _ => self.facing = Right,
            },
            Down => match dir {
                Left => self.facing = Right,
                _ => self.facing = Left,
            },
        }
    }

    fn walk(&mut self) -> WalkRes {
        let mut pos = self.pos;
        match self.facing {
            Left => pos.0 -= 1,
            Right => pos.0 += 1,
            Up => pos.1 -= 1,
            Down => pos.1 += 1,
        };
        if pos.0 > 49 || pos.1 > 49 {
            WalkRes::Wrap
        } else if self.grid[pos.1][pos.0] == Wall {
            WalkRes::Wall
        } else {
            self.pos = pos;
            WalkRes::Ok
        }
    }

    fn current_sqr(&self) -> Square {
        self.grid[self.pos.1][self.pos.0]
    }
}

struct Walker3D {
    walkers: HashMap<usize, SideWalker>,
    current_id: usize,
    instructions: Vec<Instruction>,
}

impl Walker3D {
    fn parse() -> Walker3D {
        let input = get_input(22);
        let (grid_lines, instructions) = input.split_once("\n\n").unwrap();
        let instructions = many1(Instruction::parse)(instructions).unwrap().1;
        let lines = grid_lines
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        ' ' => Null,
                        '.' => Empty,
                        _ => Wall,
                    })
                    .collect_vec()
            })
            .collect_vec();
        let mut grids = [([[Empty; 50]; 50], (0, 0)); 6];
        let mut id = 0;
        for y_start in (0..).step_by(50).take_while(|&y| y < lines.len()) {
            for x_start in (0..).step_by(50).take_while(|&x| x < lines[y_start].len()) {
                if lines[y_start][x_start] == Null {
                    continue;
                }
                for y in (y_start..).take(50) {
                    for x in (x_start..).take(50) {
                        if lines[y][x] == Wall {
                            grids[id].0[y - y_start][x - x_start] = Wall;
                        }
                    }
                }
                grids[id].1 = (x_start, y_start);
                id += 1;
            }
        }
        let mut walkers = HashMap::new();
        for (id, (grid, offset)) in grids.into_iter().enumerate() {
            let x = grid[0].iter().position(|&sqr| sqr == Empty).unwrap();
            walkers.insert(
                id,
                SideWalker {
                    grid,
                    facing: Right,
                    pos: (x, 0),
                    offset,
                },
            );
        }
        Walker3D {
            walkers,
            current_id: 0,
            instructions,
        }
    }

    fn walker_mut(&mut self, id: usize) -> &mut SideWalker {
        self.walkers.get_mut(&id).unwrap()
    }

    fn current_walker(&mut self) -> &mut SideWalker {
        self.walker_mut(self.current_id)
    }

    fn walk(&mut self, dist: usize) {
        for _ in 0..dist {
            let walker = self.current_walker();
            match walker.walk() {
                WalkRes::Ok => continue,
                WalkRes::Wall => return,
                _ => {}
            }
            let pos = walker.pos;
            let current_dir = walker.facing;
            let (next_id, next_dir) = NEXT[&self.current_id][&current_dir];
            let next_walker = self.walker_mut(next_id);
            let next_pos = match (current_dir, next_dir) {
                (Left, Left) | (Right, Right) => (None, Some(pos.1)),
                (Left, Right) | (Right, Left) => (None, Some(49 - pos.1)),
                (Up, Up) | (Down, Down) => (Some(pos.0), None),
                (Up, Down) | (Down, Up) => (Some(49 - pos.0), None),
                (Left, Down) | (Right, Up) => (Some(pos.1), None),
                (Left, Up) | (Right, Down) => (Some(49 - pos.1), None),
                (Up, Right) | (Down, Left) => (None, Some(pos.0)),
                (Up, Left) | (Down, Right) => (None, Some(49 - pos.0)),
            };
            let next_pos = match next_dir {
                Left | Up => next_pos.map(|i| i.unwrap_or(49)),
                _ => next_pos.map(|i| i.unwrap_or(0)),
            };
            next_walker.pos = next_pos;
            if next_walker.current_sqr() == Wall {
                return;
            }
            next_walker.facing = next_dir;
            self.current_id = next_id;
        }
    }

    fn abs_position(&self) -> (usize, usize, Direction) {
        let walker = &self.walkers[&self.current_id];
        (
            walker.pos.0 + walker.offset.0 + 1,
            walker.pos.1 + walker.offset.1 + 1,
            walker.facing,
        )
    }

    fn simulate(&mut self) {
        let instructions = self.instructions.clone();
        for i in instructions {
            match i {
                Turn(dir) => self.current_walker().turn(dir),
                Walk(dist) => self.walk(dist),
            }
        }
    }

    fn calc_password(&self) -> usize {
        let (x, y, facing) = self.abs_position();
        y * 1000 + x * 4 + facing as usize
    }
}

fn main() {
    let mut walker = Walker3D::parse();
    walker.simulate();
    println!("Solution to problem 2: {}", walker.calc_password());
}
