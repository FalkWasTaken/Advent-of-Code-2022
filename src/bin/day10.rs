
use Instruction::*;

enum Instruction {
    Noop,
    Addx(isize)
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        if s == "noop" {
            Noop
        } else {
            let (inst, val) = s.split_once(" ").unwrap();
            match inst {
                
            }
        }
    }
}


struct CPU {
    x: isize,
    ops: Vec<(isize, usize)>
}

impl CPU {
    fn new() -> CPU {
        CPU {
            x: 1,
            ops: vec![]
        }
    }

    fn execute(&mut self, inst: Instruction) {
        match inst {
            Noop => {},
            Addx(v) => self.ops.push((v, 2)),
        }
        for op in self.ops.iter_mut() {
            op.1 -= 1;
            if op.1 == 0 {
                self.x += op.0;
            }
        }
        self.ops.retain(|op| op.1 > 0);
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/day10.in").unwrap();

}