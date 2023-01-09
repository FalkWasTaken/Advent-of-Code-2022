use itertools::Itertools;
use std::collections::VecDeque;
use std::ops::Index;
use utils::*;

struct GridSquare {
    height: usize,
    start: bool,
    end: bool,
}

impl From<char> for GridSquare {
    fn from(c: char) -> Self {
        match c {
            'S' => GridSquare {
                height: 0,
                start: true,
                end: false,
            },
            'E' => GridSquare {
                height: 'z' as usize - 'a' as usize,
                start: false,
                end: true,
            },
            _ => GridSquare {
                height: c as usize - 'a' as usize,
                start: false,
                end: false,
            },
        }
    }
}

type Pos = (usize, usize);

struct Grid {
    vec: Vec<Vec<GridSquare>>,
    rev: bool,
}

impl Index<usize> for Grid {
    type Output = Vec<GridSquare>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl Index<Pos> for Grid {
    type Output = GridSquare;
    fn index(&self, index: Pos) -> &Self::Output {
        &self.vec[index.0][index.1]
    }
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        Grid {
            vec: s
                .lines()
                .map(|l| l.chars().map(GridSquare::from).collect())
                .collect(),
            rev: false,
        }
    }
}

impl Grid {
    fn len(&self) -> (usize, usize) {
        (self.vec[0].len(), self.vec.len())
    }

    fn start_pos(&self) -> Pos {
        (0..self.len().1)
            .cartesian_product(0..self.len().0)
            .find(|&pos| self[pos].start)
            .expect("No start point defined!")
    }

    fn accept_next(&self, from: Pos, to: Pos) -> bool {
        match self.rev {
            false => self[to].height <= self[from].height + 1,
            true => self[to].height >= self[from].height - 1,
        }
    }

    fn get_neighbors(&self, (i, j): Pos) -> impl Iterator<Item = Pos> + '_ {
        (1.max(i) - 1..=(self.len().1 - 1).min(i + 1))
            .map(move |i2| (i2, j))
            .chain((1.max(j) - 1..=(self.len().0 - 1).min(j + 1)).map(move |j2| (i, j2)))
            .filter(move |&to| self.accept_next((i, j), to))
    }

    fn bfs(&self) -> usize {
        let start_pos = self.start_pos();
        let mut visited = vec![vec![false; self.len().0]; self.len().1];
        let mut queue = VecDeque::from([(start_pos, 0)]);
        while let Some((pos, depth)) = queue.pop_front() {
            if visited[pos.0][pos.1] {
                continue;
            }
            visited[pos.0][pos.1] = true;
            for neighbor in self.get_neighbors(pos) {
                if self[neighbor].end {
                    return depth + 1;
                }
                queue.push_back((neighbor, depth + 1));
            }
        }
        panic!("Could not find a path to the end!");
    }

    fn reverse(&mut self) {
        for row in &mut self.vec {
            for sqr in row {
                sqr.start = sqr.end;
                sqr.end = sqr.height == 0;
            }
        }
        self.rev = true;
    }
}

fn main() {
    let input = get_input(12);
    let mut grid = Grid::from(input.as_str());
    println!("Solution to problem 1: {}", grid.bfs());
    grid.reverse();
    println!("Solution to problem 2: {}", grid.bfs());
}
