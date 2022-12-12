use std::collections::VecDeque;
use std::ops::{Deref, Index};
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

impl Deref for Grid {
    type Target = Vec<Vec<GridSquare>>;
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
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
    fn start_pos(&self) -> Pos {
        (0..self.len())
            .flat_map(|i| (0..self[0].len()).map(move |j| (i, j)))
            .find(|&pos| self[pos].start)
            .unwrap()
    }

    fn valid_height(&self, from: usize, to: usize) -> bool {
        match self.rev {
            false => to <= from + 1,
            true => to >= from - 1,
        }
    }

    fn get_neighbors(&self, (i, j): Pos) -> Vec<Pos> {
        let mut res = Vec::with_capacity(4);
        let height = self[i][j].height;
        if i > 0 {
            res.push((i - 1, j));
        }
        if i < self.len() - 1 {
            res.push((i + 1, j));
        }
        if j > 0 {
            res.push((i, j - 1));
        }
        if j < self[0].len() - 1 {
            res.push((i, j + 1));
        }
        res.retain(|&pos| self.valid_height(height, self[pos].height));
        res
    }

    fn bfs(&self) -> usize {
        let start_pos = self.start_pos();
        let mut visited = vec![vec![false; self[0].len()]; self.len()];
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
