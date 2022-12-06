type Stacks = Vec<Vec<char>>;

struct Move {
    from: usize,
    to: usize,
    size: usize,
}

impl From<&str> for Move {
    fn from(line: &str) -> Self {
        let vals: Vec<_> = line.split(",").map(|n| n.parse().unwrap()).collect();
        Move {
            from: vals[1] - 1,
            to: vals[2] - 1,
            size: vals[0],
        }
    }
}

fn output_string(stacks: Stacks) -> String {
    stacks.iter().filter_map(|s| s.last()).collect()
}

fn process1(mut stacks: Stacks, moves: &Vec<Move>) {
    for &Move { from, to, size } in moves {
        for _ in 0..size {
            let c = stacks[from].pop().unwrap();
            stacks[to].push(c);
        }
    }
    println!("{}", output_string(stacks))
}

fn process2(mut stacks: Vec<Vec<char>>, moves: &Vec<Move>) {
    for &Move { from, to, size } in moves {
        let cutoff = stacks[from].len() - size;
        let mut crates = stacks[from].split_off(cutoff);
        stacks[to].append(&mut crates);
    }
    println!("{}", output_string(stacks))
}

fn main() {
    let input = std::fs::read_to_string("inputs/day5.in").unwrap();
    let (stacks, moves) = input.split_once("\n\n").unwrap();
    let stacks = stacks.lines().map(|l| l.chars().collect()).collect();
    let moves = moves.lines().map(Move::from).collect();
    //process1(stacks, &moves);
    process2(stacks, &moves);
}
