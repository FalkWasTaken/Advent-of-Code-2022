use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    sequence::{preceded, terminated},
    IResult,
};
use utils::*;

type MonkeyID<'a> = &'a str;
type MonkeyPair<'a> = (MonkeyID<'a>, MonkeyID<'a>);

enum Result {
    Val(i64),
    Eq(bool)
}

impl Result {
    fn val(&self) -> i64 {
        match self {
            &Result::Val(v) => v,
            _ => panic!()
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Eq
}

impl Operator {
    fn parse(s: &str) -> IResult<&str, Operator> {
        let (s, op) = alt((tag("+"), tag("-"), tag("/"), tag("*")))(s)?;
        Ok((
            s,
            match op {
                "+" => Operator::Add,
                "-" => Operator::Sub,
                "*" => Operator::Mul,
                _ => Operator::Div,
            },
        ))
    }
}

#[derive(Debug, Clone, Copy)]
enum Job<'a> {
    Val(i64),
    Equation(MonkeyPair<'a>, Operator),
}

fn parse_id(s: &str) -> IResult<&str, MonkeyID> {
    nom::character::complete::alpha1(s)
}

fn parse_equation(s: &str) -> IResult<&str, Job> {
    let (s, id1) = terminated(parse_id, tag(" "))(s)?;
    let (s, op) = Operator::parse(s)?;
    let (s, id2) = preceded(tag(" "), parse_id)(s)?;
    Ok((s, Job::Equation((id1, id2), op)))
}

fn parse_val(s: &str) -> IResult<&str, Job> {
    let (s, val) = nom::character::complete::i64(s)?;
    Ok((s, Job::Val(val)))
}

impl Job<'_> {
    fn parse(s: &str) -> IResult<&str, Job> {
        nom::branch::alt((parse_val, parse_equation))(s)
    }

    fn val(&self) -> i64 {
        match self {
            Job::Val(v) => *v,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Monkey<'a> {
    id: MonkeyID<'a>,
    job: Job<'a>,
}

impl Monkey<'_> {
    fn parse(s: &str) -> IResult<&str, Monkey> {
        let (s, id) = terminated(parse_id, tag(": "))(s)?;
        let (s, job) = Job::parse(s)?;
        Ok((s, Monkey { id, job }))
    }
}

#[derive(Debug, Clone)]
struct Monkeys<'a>(HashMap<&'a str, Monkey<'a>>);

impl Monkeys<'_> {
    fn calc_dfs(&mut self, from: &str, ignore: Option<&str>) -> Option<Result> {
        if ignore.is_some() && ignore.unwrap() == from {
            return None;
        }
        let Job::Equation((id1, id2), operator) = self.0[from].job else {
            return Some(Result::Val(self.0[from].job.val()));
        };
        let r1 = self.calc_dfs(id1, ignore);
        let r2 = self.calc_dfs(id2, ignore);
        let v1 = r1?.val();
        let v2 = r2?.val();
        let res = match operator {
            Operator::Add => Result::Val(v1 + v2),
            Operator::Sub => Result::Val(v1 - v2),
            Operator::Mul => Result::Val(v1 * v2),
            Operator::Div => Result::Val(v1 / v2),
            Operator::Eq => Result::Eq(v1 == v2)
        };
        if let Result::Val(v) = res {
            self.0.get_mut(from).unwrap().job = Job::Val(v);
        }
        Some(res)
    }
}

fn solve1(mut monkeys: Monkeys) {
    let res = monkeys.calc_dfs("root", None).unwrap().val();
    println!("Solution to problem 1: {res}");
}

fn solve2(mut monkeys: Monkeys) {
    let Job::Equation((id1, id2), _) = monkeys.0["root"].job else {
        return;
    };
    monkeys.0.get_mut("root").unwrap().job = Job::Equation((id1, id2), Operator::Eq);
    monkeys.calc_dfs("root", Some("humn"));
    for x in 395_228_867_000_0.. {
        let mut m2 = monkeys.clone();
        m2.0.get_mut("humn").unwrap().job = Job::Val(x);
        if matches!(m2.calc_dfs("root", None).unwrap(), Result::Eq(true)) {
            println!("Solution to problem 2: {x}");
            return
        }
        if let Job::Equation((id1, id2), _) = m2.0["root"].job {
            let l = m2.0[id1].job.val();
            let r = m2.0[id2].job.val();
            println!("{l} = {r}, diff: {}", l - r);
        }
    }
}

fn main() {
    let input = input!();
    let monkeys = Monkeys(
        input
            .lines()
            .flat_map(Monkey::parse)
            .map(|r| (r.1.id, r.1))
            .collect(),
    );
    solve1(monkeys.clone());
    solve2(monkeys);
}
