use utils::get_input;

#[derive(PartialEq, Eq, Ord, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl Shape {
    fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        }
    }

    fn score_against(&self, other: Shape) -> usize {
        if *self == other {
            self.score() + 3
        } else if *self > other {
            self.score() + 6
        } else {
            self.score()
        }
    }

    fn lose(&self) -> usize {
        match self {
            Self::Rock => Self::Scissor.score(),
            Self::Paper => Self::Rock.score(),
            Self::Scissor => Self::Paper.score(),
        }
    }

    fn win(&self) -> usize {
        match self {
            Self::Rock => Self::Paper.score() + 6,
            Self::Paper => Self::Scissor.score() + 6,
            Self::Scissor => Self::Rock.score() + 6,
        }
    }

    fn draw(&self) -> usize {
        self.score() + 3
    }

    fn with_outcome(&self, outcome: &str) -> usize {
        match outcome {
            "X" => self.lose(),
            "Y" => self.draw(),
            "Z" => self.win(),
            _ => panic!(),
        }
    }
}

impl From<&str> for Shape {
    fn from(c: &str) -> Self {
        match c {
            "X" | "A" => Self::Rock,
            "Y" | "B" => Self::Paper,
            "Z" | "C" => Self::Scissor,
            _ => panic!(),
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        }
        match self {
            Self::Rock => {
                if matches!(other, Self::Paper) {
                    Some(std::cmp::Ordering::Less)
                } else {
                    Some(std::cmp::Ordering::Greater)
                }
            }
            Self::Paper => {
                if matches!(other, Self::Scissor) {
                    Some(std::cmp::Ordering::Less)
                } else {
                    Some(std::cmp::Ordering::Greater)
                }
            }
            Self::Scissor => {
                if matches!(other, Self::Rock) {
                    Some(std::cmp::Ordering::Less)
                } else {
                    Some(std::cmp::Ordering::Greater)
                }
            }
        }
    }
}

fn main() {
    let res = get_input(2)
        .lines()
        .map(|v| Shape::from(&v[0..1]).with_outcome(&v[2..3]))
        .sum::<usize>();
    println!("{res}");
}
