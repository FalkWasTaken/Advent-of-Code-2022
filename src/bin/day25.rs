use utils::*;

fn to_digit(c: char) -> isize {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '-' => -1,
        '=' => -2,
        _ => panic!(),
    }
}

fn to_char(d: &isize) -> char {
    match d {
        0 => '0',
        1 => '1',
        2 => '2',
        -1 => '-',
        -2 => '=',
        _ => panic!(),
    }
}

fn pow(exp: usize) -> isize {
    5_isize.pow(exp as u32)
}

struct SNAFU(Vec<isize>);

impl SNAFU {
    fn from_str(s: &str) -> SNAFU {
        SNAFU(s.chars().rev().map(to_digit).collect())
    }

    fn to_string(&self) -> String {
        self.0.iter().rev().map(to_char).collect()
    }

    fn to_decimal(&self) -> isize {
        self.0
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &d)| acc + d * pow(i))
    }

    fn from_decimal(num: isize) -> SNAFU {
        let num_digits = (num as f64).log(5.0).ceil() as usize;
        let mut snafu = SNAFU(vec![0; num_digits]);
        snafu.from_dec_rec(num, num_digits - 1, 0);
        snafu
    }

    fn from_dec_rec(&mut self, num: isize, current_digit: usize, total: isize) -> bool {
        let exp = pow(current_digit);
        let values = [-2, -1, 0, 1, 2];
        let values = [
            values.iter().find(|&&v| total + v * exp == num),
            values.iter().find(|&&v| total + v * exp > num),
            values.iter().rev().find(|&&v| total + v * exp < num),
        ];
        for &v in values.into_iter().filter_map(|v| v) {
            let total = total + v * exp;
            self.0[current_digit] = v;
            if current_digit == 0 {
                if total == num {
                    return true;
                }
            } else if self.from_dec_rec(num, current_digit - 1, total) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let res: isize = input!()
        .lines()
        .map(SNAFU::from_str)
        .map(|n| n.to_decimal())
        .sum();
    let snafu = SNAFU::from_decimal(res);
    println!("input: {}", snafu.to_string());
}
