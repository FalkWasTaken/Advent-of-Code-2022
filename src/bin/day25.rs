use std::ops::Add;

use itertools::EitherOrBoth::*;
use itertools::Itertools;
use utils::*;

fn to_digit(c: char) -> i8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '-' => -1,
        '=' => -2,
        _ => panic!(),
    }
}

fn to_char(d: &i8) -> char {
    match d {
        0 => '0',
        1 => '1',
        2 => '2',
        -1 => '-',
        -2 => '=',
        _ => panic!(),
    }
}

struct SNAFU(Vec<i8>);

impl SNAFU {
    fn from_str(s: &str) -> SNAFU {
        SNAFU(s.chars().rev().map(to_digit).collect())
    }

    fn to_string(&self) -> String {
        self.0.iter().rev().map(to_char).collect()
    }
}

impl Add<SNAFU> for SNAFU {
    type Output = SNAFU;
    fn add(self, rhs: SNAFU) -> Self::Output {
        let mut rem = 0;
        let mut vec = self
            .0
            .into_iter() 
            .zip_longest(rhs.0)
            .map(|t| {
                let res = match t {
                    Both(x, y) => x + y,
                    Left(x) | Right(x) => x,
                } + rem;
                rem = (res < -2 || res > 2) as i8 * res.signum();
                -5 * rem + res
            })
            .collect_vec();
        if rem != 0 {
            vec.push(rem);
        }
        SNAFU(vec)
    }
}

fn main() {
    let res = input!()
        .lines()
        .map(SNAFU::from_str)
        .reduce(|sum, n| sum + n)
        .unwrap();
    println!("Solution: {}", res.to_string());
}
