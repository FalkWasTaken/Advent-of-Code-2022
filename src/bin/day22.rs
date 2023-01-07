use nom::{branch::alt, bytes::complete::tag, character::complete::digit1, multi::many1, IResult};
use utils::*;

use Direction::*;
use Instruction::*;
use Square::*;

#[derive(Clone, Copy)]
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

struct Walker {
    instructions: Vec<Instruction>,
    grid: Vec<Vec<Square>>,
    bounds_x: Vec<[usize; 2]>,
    bounds_y: Vec<[usize; 2]>,
    facing: Direction,
    x: usize,
    y: usize,
}

impl Walker {
    fn parse() -> Walker {
        let input = input!();
        let (grid_lines, instructions) = input.split_once("\n\n").unwrap();
        let grid_lines: Vec<_> = grid_lines.lines().collect();
        let instructions = many1(Instruction::parse)(instructions).unwrap().1;
        let max_x = grid_lines.iter().max_by_key(|&l| l.len()).unwrap().len();
        let mut grid = vec![vec![Null; max_x]; grid_lines.len()];
        for (y, line) in grid_lines.into_iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => grid[y][x] = Empty,
                    '#' => grid[y][x] = Wall,
                    _ => {}
                }
            }
        }
        let mut bounds_x = vec![[0, 0]; grid.len()];
        let mut bounds_y = vec![[0, 0]; max_x];
        'outer: for y in 0..grid.len() {
            let mut found_lower = false;
            for x in 0..max_x {
                if found_lower && grid[y][x] == Null {
                    bounds_x[y][1] = x - 1;
                    continue 'outer;
                }
                if !found_lower && grid[y][x] != Null {
                    bounds_x[y][0] = x;
                    found_lower = true;
                }
            }
            bounds_x[y][1] = max_x - 1;
        }
        'outer: for x in 0..max_x {
            let mut found_lower = false;
            for y in 0..grid.len() {
                if found_lower && grid[y][x] == Null {
                    bounds_y[x][1] = y - 1;
                    continue 'outer;
                }
                if !found_lower && grid[y][x] != Null {
                    bounds_y[x][0] = y;
                    found_lower = true;
                }
            }
            bounds_y[x][1] = grid.len() - 1;
        }
        Walker {
            x: bounds_x[0][0],
            y: 0,
            instructions,
            grid,
            bounds_x,
            bounds_y,
            facing: Right,
        }
    }

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

    fn walk(&mut self, dist: usize) {
        let mut pos = [self.x, self.y];
        let bounds_x = self.bounds_x[pos[1]];
        let bounds_y = self.bounds_y[pos[0]];
        for _ in 0..dist {
            match self.facing {
                Left => {
                    pos[0] = if pos[0] == bounds_x[0] {
                        bounds_x[1]
                    } else {
                        pos[0] - 1
                    }
                }
                Right => {
                    pos[0] = if pos[0] == bounds_x[1] {
                        bounds_x[0]
                    } else {
                        pos[0] + 1
                    }
                }
                Up => {
                    pos[1] = if pos[1] == bounds_y[0] {
                        bounds_y[1]
                    } else {
                        pos[1] - 1
                    };
                }
                Down => {
                    pos[1] = if pos[1] == bounds_y[1] {
                        bounds_y[0]
                    } else {
                        pos[1] + 1
                    }
                }
            };
            if self.grid[pos[1]][pos[0]] == Wall {
                return;
            }
            self.x = pos[0];
            self.y = pos[1];
        }
    }

    fn simulate(&mut self) {
        self.facing = Right;
        let instructions = self.instructions.clone();
        self.x = self.bounds_x[0][0];
        self.y = 0;
        for i in instructions {
            match i {
                Turn(dir) => self.turn(dir),
                Walk(dist) => self.walk(dist),
            }
        }
    }

    fn calc_password(&self) -> usize {
        (self.y + 1) * 1000 + (self.x + 1) * 4 + self.facing as usize
    }
}

fn main() {
    let mut walker = Walker::parse();
    walker.simulate();
    println!("Solution to problem 1: {}", walker.calc_password());
}
