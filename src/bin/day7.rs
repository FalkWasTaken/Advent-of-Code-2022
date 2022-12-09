use std::{collections::HashMap, iter::once};

const TOTAL_SPACE: usize = 70_000_000;
const REQ_SPACE: usize = 30_000_000;

trait ExtendIter: Iterator + Sized {
    fn pop(&mut self) -> Self::Item {
        self.next().unwrap()
    }

    fn collect_vec(self) -> Vec<Self::Item> {
        self.collect()
    }
}

impl<I: Iterator> ExtendIter for I {}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls(Vec<usize>),
}

impl From<&str> for Command {
    fn from(c: &str) -> Self {
        let mut lines = c.trim().lines();
        let command = lines.pop();
        match command {
            "ls" => Command::Ls(
                lines
                    .map(|l| l.split_whitespace())
                    .filter_map(|mut r| r.pop().parse().ok())
                    .collect(),
            ),
            _ => Command::Cd(command.split_once(" ").unwrap().1.into()),
        }
    }
}

#[derive(Debug)]
struct Directory {
    len: usize,
    content: HashMap<String, Directory>,
}

impl Default for Directory {
    fn default() -> Self {
        Directory {
            len: 0,
            content: HashMap::new(),
        }
    }
}

impl Directory {
    fn traverse_mut(&mut self, path: &[String]) -> &mut Directory {
        if path.len() == 0 {
            return self;
        }
        self.content
            .get_mut(&path[0])
            .unwrap()
            .traverse_mut(&path[1..])
    }

    fn update_sizes(&mut self) -> usize {
        self.len += self
            .content
            .values_mut()
            .map(|d| d.update_sizes())
            .sum::<usize>();
        self.len
    }

    fn iter(&self) -> impl Iterator<Item = &Directory> {
        once(self).chain(self.content.values().flat_map(|d| d.iter().collect_vec()))
    }
}

struct FileSystem {
    file_tree: Directory,
    current_path: Vec<String>,
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            file_tree: Directory::default(),
            current_path: vec![],
        }
    }

    fn apply_command(&mut self, command: Command) {
        match command {
            Command::Cd(dir) => {
                if dir == ".." {
                    self.current_path.pop();
                } else {
                    let d = self.file_tree.traverse_mut(&self.current_path);
                    d.content.entry(dir.clone()).or_default();
                    self.current_path.push(dir);
                }
            }
            Command::Ls(res) => {
                self.file_tree.traverse_mut(&self.current_path).len +=
                    res.into_iter().sum::<usize>();
            }
        }
    }
}

fn solve1(fs: &FileSystem) {
    let res: usize = fs
        .file_tree
        .iter()
        .map(|d| d.len)
        .filter(|&size| size <= 100000)
        .sum();
    println!("{res}")
}

fn solve2(fs: &FileSystem) {
    let target = fs.file_tree.len + REQ_SPACE - TOTAL_SPACE;
    let mut res = fs.file_tree.iter().map(|d| d.len).collect_vec();
    res.sort();
    let res = res.iter().find(|&&s| s >= target).unwrap();
    println!("{res}")
}

fn main() {
    let input = std::fs::read_to_string("inputs/day7.in").unwrap();
    let mut fs = FileSystem::new();
    for command in input.split("$").skip(1) {
        let command = Command::from(command);
        fs.apply_command(command);
    }
    fs.file_tree.update_sizes();
    solve1(&fs);
    solve2(&fs);
}
