use std::{cmp::Ordering, collections::VecDeque};

use utils::*;

use Packet::*;

type TokenStream = VecDeque<Token>;

#[derive(PartialEq, Eq, Ord, Clone)]
enum Packet {
    Value(usize),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value(v1), Value(v2)) => v1.partial_cmp(v2),
            (List(l1), List(l2)) => l1.partial_cmp(l2),
            (Value(_), _) => self.to_list().partial_cmp(other),
            (_, Value(_)) => self.partial_cmp(&other.to_list()),
        }
    }
}

impl Packet {
    fn to_list(&self) -> Packet {
        List(vec![self.clone()])
    }

    fn parse(line: &str) -> Packet {
        let mut tokens = tokenize_line(line);
        assert!(matches!(tokens.pop_front(), Some(Token::Left)));
        Packet::parse_rec(&mut tokens)
    }

    fn parse_rec(tokens: &mut TokenStream) -> Packet {
        let mut content = Vec::new();
        while let Some(token) = tokens.pop_front() {
            match token {
                Token::Left => content.push(Packet::parse_rec(tokens)),
                Token::Right => return List(content),
                Token::Value(v) => content.push(Value(v)),
            }
        }
        panic!("Missing closing bracket!");
    }
}

enum Token {
    Left,
    Right,
    Value(usize),
}

fn tokenize_line(l: &str) -> TokenStream {
    let mut tokens = VecDeque::new();
    let mut val_buffer = String::new();
    for c in l.chars() {
        match c {
            '[' | ']' | ',' => {
                if !val_buffer.is_empty() {
                    let val = std::mem::take(&mut val_buffer);
                    tokens.push_back(Token::Value(val.parse().unwrap()));
                }
                match c {
                    '[' => tokens.push_back(Token::Left),
                    ']' => tokens.push_back(Token::Right),
                    _ => {}
                }
            }
            _ => val_buffer.push(c),
        }
    }
    tokens
}

fn solve1(packets: &Vec<Packet>) {
    let res = packets
        .chunks(2)
        .enumerate()
        .filter(|(_, p)| p[0] <= p[1])
        .map(|(i, _)| i + 1)
        .sum::<usize>();
    println!("Solution to problem 1: {res}");
}

fn solve2(mut packets: Vec<Packet>) {
    let separators = [Packet::parse("[[2]]"), Packet::parse("[[6]]")];
    packets.extend_from_slice(&separators);
    packets.sort();
    let key = packets
        .into_iter()
        .enumerate()
        .filter(|(_, p)| separators.contains(p))
        .map(|(i, _)| i + 1)
        .product::<usize>();
    println!("Solution to problem 2: {key}");
}

fn main() {
    let packets = input!()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| Packet::parse(l))
        .collect();
    solve1(&packets);
    solve2(packets);
}
