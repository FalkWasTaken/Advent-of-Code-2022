use utils::*;

const DECRYPT_KEY: isize = 811_589_153;

#[derive(Clone)]
struct Elem {
    init: usize,
    val: isize,
}

#[derive(Clone)]
struct File {
    len: isize,
    list: Vec<Elem>,
    current: usize,
}

impl File {
    fn new(list: Vec<Elem>) -> File {
        File {
            len: list.len() as isize,
            list,
            current: 0,
        }
    }

    fn find_elem(&self, f: impl Fn(&(usize, &Elem)) -> bool) -> Option<(usize, &Elem)> {
        self.list.iter().enumerate().find(f)
    }

    fn align(&mut self) {
        let (i, _) = self.find_elem(|(_, e)| e.val == 0).unwrap();
        self.list.rotate_left(i);
    }

    fn mix(&mut self) {
        while let Some((i, e)) = self.find_elem(|(_, e)| e.init == self.current) {
            let dest = ((i as isize + e.val).rem_euclid(self.len - 1)) as usize;
            if dest < i {
                self.list[dest..=i].rotate_right(1);
            } else if dest > i {
                self.list[i..=dest].rotate_left(1);
            }
            self.current += 1;
        }
        self.current = 0;
    }

    fn get_coords(&self) -> isize {
        [1, 2, 3]
            .iter()
            .map(|&i| self.list[(1000 * i) % self.list.len()].val)
            .sum()
    }
}

fn solve1(mut file: File) {
    file.mix();
    file.align();
    let res = file.get_coords();
    println!("Solution to problem 1: {res}");
}

fn solve2(mut file: File) {
    for e in &mut file.list {
        e.val *= DECRYPT_KEY;
    }
    for _ in 0..10 {
        file.mix();
    }
    file.align();
    let res = file.get_coords();
    println!("Solution to problem 2: {res}");
}

fn main() {
    let file = File::new(
        input!()
            .lines()
            .flat_map(|l| l.parse())
            .enumerate()
            .map(|(i, val)| Elem { init: i, val })
            .collect(),
    );
    solve1(file.clone());
    solve2(file);
}
