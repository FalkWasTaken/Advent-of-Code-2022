use num::Integer;
use utils::*;
use Operation::*;

#[derive(Clone)]
enum Operation {
    Add(usize),
    Mul(usize),
    AddSelf,
    MulSelf,
}

impl Operation {
    fn eval(&self, old: usize) -> usize {
        match self {
            Add(x) => old + x,
            Mul(x) => old * x,
            AddSelf => old + old,
            MulSelf => old * old,
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: usize,
    targets: [usize; 2],
    counter: usize,
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let mut lines = s.lines().skip(1);
        let items = lines
            .pop()
            .trim()
            .trim_start_matches("Starting items: ")
            .split(", ")
            .flat_map(|s| s.parse())
            .collect();
        let op_tokens = lines
            .pop()
            .trim()
            .trim_start_matches("Operation: new = old ")
            .split_once(" ")
            .unwrap();
        let operation = match op_tokens {
            ("+", "old") => AddSelf,
            ("*", "old") => MulSelf,
            ("+", val) => Add(val.parse().unwrap()),
            ("*", val) => Mul(val.parse().unwrap()),
            _ => panic!("Invalid operation!"),
        };
        let mut pick_last = || lines.pop().split_whitespace().pop_back().parse().unwrap();
        let test = pick_last();
        let targets = [pick_last(), pick_last()];
        Monkey {
            items,
            operation,
            test,
            targets,
            counter: 0,
        }
    }
}

impl Monkey {
    fn make_turn(&mut self, divider: usize) -> Vec<(usize, usize)> {
        self.counter += self.items.len();
        self.items
            .drain(0..)
            .map(|item| {
                let new_worry = self.operation.eval(item) / divider;
                match new_worry % self.test {
                    0 => (self.targets[0], new_worry),
                    _ => (self.targets[1], new_worry),
                }
            })
            .collect()
    }
}

fn monkey_business(mut monkeys: Vec<Monkey>) -> usize {
    monkeys.sort_by_key(|m| std::cmp::Reverse(m.counter));
    monkeys[0].counter * monkeys[1].counter
}

fn solve1(mut monkeys: Vec<Monkey>) {
    for _ in 0..20 {
        for id in 0..monkeys.len() {
            let items = monkeys[id].make_turn(3);
            for (target_id, item) in items {
                monkeys[target_id].items.push(item);
            }
        }
    }
    println!("Solution for problem 1: {}", monkey_business(monkeys));
}

fn solve2(mut monkeys: Vec<Monkey>) {
    let lcm = monkeys.iter().map(|m| m.test).reduce(|acc, t| acc.lcm(&t)).unwrap();
    for _ in 0..10_000 {
        for id in 0..monkeys.len() {
            let items = monkeys[id].make_turn(1);
            for (target_id, item) in items {
                monkeys[target_id].items.push(item % lcm);
            }
        }
    }
    println!("Solution for problem 2: {}", monkey_business(monkeys));
}

fn main() {
    let input = get_input(11);
    let monkeys = input.split("\n\n").map(Monkey::from).collect_vec();
    solve1(monkeys.clone());
    solve2(monkeys);
}
