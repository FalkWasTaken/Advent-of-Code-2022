use itertools::Itertools;
use Direction::*;

const TEST: &str = 
"30373
25512
65332
33549
35390";

fn print_visited(v: &Vec<Vec<bool>>) {
    println!("Visited trees:");
    for row in v {
        for &b in row {
            print!("{}", b as u32);
        }
        println!()
    }
}

fn map_tup<I, T>(t: (I, I), f: fn(I) -> T) -> (T, T) {
    (f(t.0), f(t.1))
}

enum Direction {
    Up,
    Down,
    Left,
    Right
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
        },
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


fn solve1(grid: &Vec<Vec<u32>>) {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    for direction in [Up, Down, Left, Right] {
        fill_visited(grid, &mut visited, direction);
    }
    // print_visited(&visited);
    let res: u32 = visited.iter().flat_map(|row| row.iter().map(|&b| b as u32)).sum();
    println!("Answer to problem 1: {res}");
}


fn sovle2(grid: &Vec<Vec<u32>>) {
    todo!()
}

fn main() {
    let input = std::fs::read_to_string("inputs/day8.in").unwrap();
    // let input = TEST;
    let grid: Vec<Vec<u32>> = input.lines().map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect()).collect();
    solve1(&grid);
    sovle2(&grid);
}