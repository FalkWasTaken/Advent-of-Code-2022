use Instruction::*;
use utils::get_input;

enum Instruction {
    Noop,
    Addx(isize)
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        if  let Some((inst, val)) = s.split_once(" ") {
            match inst {
                "addx"  => Addx(val.parse().expect("Invalid value in addx operation!")),
                _ => panic!("Invalid instruction!")
            }
        } else if s == "noop" {
            Noop
        } else {
            panic!("Invalid instruction!");
        }
    }
}

struct CPU {
    x: isize,
    cycle: isize,
    signal_sum: isize,
    display: Vec<Vec<char>>
}

impl CPU {
    fn new() -> CPU {
        CPU {
            x: 1,
            cycle: 1,
            signal_sum: 0,
            display: vec![vec!['.'; 40]; 6]
        }
    }

    fn tick(&mut self) {
        self.check_signal();
        self.update_display();
        self.cycle += 1;
    }

    fn check_signal(&mut self) {
        if (self.cycle + 20) % 40 == 0 {
            self.signal_sum += self.x * self.cycle;
        } 
    }

    fn execute(&mut self, inst: Instruction) {
        self.tick();
        if let Addx(v) = inst {
            self.tick();
            self.x += v;
        }
    }

    fn update_display(&mut self) {
        let row = (self.cycle - 1) / 40;
        let pixel = (self.cycle - 1) % 40;
        if (pixel-1..=pixel+1).contains(&self.x) {
            self.display[row as usize][pixel as usize] = '#';
        }
    }

    fn print_display(&self) {
        println!("Display:");
        for row in &self.display {
            for pixel in row {
                print!("{pixel}");
            }
            println!();
        }
    }
}

fn main() {
    let input = get_input(10);
    let mut cpu = CPU::new();
    for inst in input.lines().map(Instruction::from) {
        cpu.execute(inst);
    }
    println!("Solution to problem 1: {}", cpu.signal_sum);
    cpu.print_display();
}