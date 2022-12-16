mod parse;

use camino::Utf8PathBuf;

use crate::day07::parse::{parse_input, Command, Entry, Line};

#[allow(dead_code)]
#[derive(Debug)]
struct Inode {
    path: Utf8PathBuf,
    size: u64,
    children: Vec<Inode>,
}

impl Inode {
    fn total_size(&self) -> u64 {
        self.size + self.children.iter().map(|c| c.total_size()).sum::<u64>()
    }

    fn all_dirs(&self) -> Box<dyn Iterator<Item = &Inode> + '_> {
        Box::new(
            std::iter::once(self).chain(
                self.children
                    .iter()
                    .filter(|c| !c.children.is_empty())
                    .flat_map(|c| c.all_dirs()),
            ),
        )
    }
}

fn create_filesystem(input_lines: Vec<Line>) -> Inode {
    let mut stack = vec![Inode {
        path: "/".into(),
        size: 0,
        children: vec![],
    }];

    for line in input_lines {
        // println!("{line:?}");
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {}
                Command::Cd(path) => match path.as_str() {
                    "/" => {}
                    ".." => {
                        let child = stack.pop();
                        stack.last_mut().unwrap().children.push(child.unwrap());
                    }
                    _ => {
                        let node = Inode {
                            path: path.clone(),
                            size: 0,
                            children: vec![],
                        };
                        stack.push(node);
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_) => {}
                Entry::File(size, path) => {
                    let node = Inode {
                        size,
                        path,
                        children: vec![],
                    };
                    stack.last_mut().unwrap().children.push(node);
                }
            },
        }
    }
    let mut root = stack.pop().unwrap();

    while let Some(mut next) = stack.pop() {
        next.children.push(root);
        root = next;
    }
    root
}

fn part1(fs: &Inode) {
    let sum = fs
        .all_dirs()
        .map(|d| d.total_size())
        .filter(|&s| s <= 100_000)
        .sum::<u64>();
    println!("Part 1: {sum}");
}

fn part2(fs: &Inode) {
    let total_space = 70_000_000_u64;
    let used_space = fs.total_size();
    let free_space = total_space.checked_sub(used_space).unwrap();
    let needed_free_space = 30_000_000_u64;
    let minimum_space_to_free = needed_free_space.checked_sub(free_space).unwrap();

    let size_to_remove = fs
        .all_dirs()
        .map(|d| d.total_size())
        .filter(|&s| s >= minimum_space_to_free)
        .min()
        .unwrap();
    println!("Part 2: {size_to_remove}");
}

pub(crate) fn run() {
    println!("\n=== Day 07 ===");
    let input_lines = parse_input("src/day07/input.txt").expect("error parsing input");
    let fs = create_filesystem(input_lines);
    // dbg!(&fs);

    part1(&fs);
    part2(&fs);
}
