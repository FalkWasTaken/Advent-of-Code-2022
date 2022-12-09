use itertools::Itertools;
use Direction::*;

fn map_tup<I, T>(t: (I, I), f: fn(I) -> T) -> (T, T) {
    (f(t.0), f(t.1))
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn fill_visited(grid: &Vec<Vec<u32>>, visited: &mut Vec<Vec<bool>>, direction: Direction) {
    let len_i = grid.len();
    let len_j = grid[0].len();
    let (range_i, range_j) = match direction {
        Up | Left => map_tup((0..len_i, 0..len_j), |r| r.rev().collect_vec()),
        Down | Right => map_tup((0..len_i, 0..len_j), |r| r.collect()),
    };
    match direction {
        Left | Right => {
            for i in range_i {
                visited[i][range_j[0]] = true;
                let mut max = grid[i][range_j[0]];
                for &j in &range_j {
                    let curr = grid[i][j];
                    if curr > max {
                        max = curr;
                        visited[i][j] = true;
                    }
                }
            }
        }
        Up | Down => {
            for j in range_j {
                visited[range_i[0]][j] = true;
                let mut max = grid[range_i[0]][j];
                for &i in &range_i {
                    let curr = grid[i][j];
                    if curr > max {
                        max = curr;
                        visited[i][j] = true;
                    }
                }
            }
        }
    }
}

fn count_trees<'a>(iter: impl Iterator<Item = &'a u32>, max_num: usize, height: u32) -> usize {
    let num_trees = iter.take_while(|&&h| h < height).count();
    num_trees + (num_trees != max_num) as usize
}

fn calc_score(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> usize {
    let height = grid[i][j];
    let max_i = grid.len() - 1;
    let max_j = grid[0].len() - 1;
    if j % max_j == 0 || i % max_i == 0 {
        return 0;
    }
    [Up, Down, Right, Left].into_iter().map(|dir| {
        match dir {
            Up => count_trees(grid[0..i].iter().rev().map(|row| &row[j]), i, height),
            Down => count_trees(grid[i + 1..].iter().map(|row| &row[j]), max_i - i, height),
            Left => count_trees(grid[i][0..j].iter().rev(), j, height),
            Right => count_trees(grid[i][j + 1..].iter(), max_j - j, height),
        }
    }).fold(1, |acc, count| acc * count)
}

fn solve1(grid: &Vec<Vec<u32>>) {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    for direction in [Up, Down, Left, Right] {
        fill_visited(grid, &mut visited, direction);
    }
    let res: u32 = visited
        .iter()
        .flat_map(|row| row.iter().map(|&b| b as u32))
        .sum();
    println!("Answer to problem 1: {res}");
}

fn sovle2(grid: &Vec<Vec<u32>>) {
    let max_score = (0..grid.len())
        .flat_map(|i| (0..grid[0].len()).map(move |j| calc_score(grid, i, j)))
        .max()
        .unwrap();
    println!("The solution to problem 2 is: {max_score}");
}

fn main() {
    let input = std::fs::read_to_string("inputs/day8.in").unwrap();
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();
    solve1(&grid);
    sovle2(&grid);
}
