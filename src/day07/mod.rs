use crate::shared::read_lines;
use std::{borrow::Borrow, cell::RefCell, rc::Rc};

#[derive(Debug, Copy, Clone, PartialEq)]
enum InodeType {
    Root,
    Dir,
    File,
}

#[derive(Debug, PartialEq)]
struct Inode {
    size: usize,
    name: String,
    kind: InodeType,
    parent: Option<Rc<RefCell<Inode>>>,
    children: Vec<Rc<RefCell<Inode>>>,
}

impl Inode {
    fn new(size: usize, name: &str, kind: InodeType) -> Self {
        Inode {
            size,
            name: name.to_string(),
            kind,
            parent: None,
            children: vec![],
        }
    }

    fn add_child(&mut self, new_node: Rc<RefCell<Inode>>) {
        self.children.push(new_node);
    }

    fn print_dfs(&self, depth: usize) {
        println!(
            "{}{}{} {}",
            " ".repeat(depth * 4),
            self.name,
            match self.kind {
                InodeType::Dir => '/',
                _ => '\0',
            },
            self.size
        );
        for child in self.children.iter() {
            child.as_ref().borrow().print_dfs(depth + 1)
        }
    }
}

fn create_ref_node(size: usize, name: &str, kind: InodeType) -> Rc<RefCell<Inode>> {
    Rc::new(RefCell::new(Inode::new(size, name, kind)))
}

fn insert_node(parent: Rc<RefCell<Inode>>, child: Rc<RefCell<Inode>>) {
    let mut mut_child = (*child).borrow_mut();
    mut_child.parent = Some(Rc::clone(&parent));
    (*parent).borrow_mut().add_child(Rc::clone(&child));
}

fn get_node(tree: Rc<RefCell<Inode>>, name: &str) -> Option<Rc<RefCell<Inode>>> {
    if (*tree).borrow().name == name {
        return Some(Rc::clone(&tree));
    } else {
        for child in (*tree).borrow().children.iter() {
            return get_node(Rc::clone(child), name);
        }
    }
    None
}

fn get_parent(node: Rc<RefCell<Inode>>) -> Option<Rc<RefCell<Inode>>> {
    if let Some(parent) = (*node).borrow().parent.as_ref() {
        get_node(Rc::clone(&node), &parent.as_ref().borrow().name)
    } else {
        None
    }
}

fn parse_input(root: Rc<RefCell<Inode>>) {
    let shell = read_lines("src/day07/input.txt")
        .expect("error reading file")
        .map(|l| {
            l.expect("error reading line")
                .split(' ')
                .map(str::to_owned)
                .collect::<Vec<_>>()
        });

    let mut current = Rc::clone(&root);

    for line in shell {
        (*current).borrow().print_dfs(0);
        let first_char = line[0].clone();
        match first_char.as_str() {
            "$" => match line[1].clone().as_str() {
                "cd" => {
                    let new_cur_node_name = line[2].clone();
                    match new_cur_node_name.as_str() {
                        ".." => {
                            let parent = get_parent(Rc::clone(&current));
                            if let Some(node) = parent {
                                current = Rc::clone(&node)
                            } else {
                                unreachable!()
                            }
                        }
                        _ => current = get_node(Rc::clone(&current), &new_cur_node_name).unwrap(),
                    }
                }
                "ls" => continue,
                _ => unreachable!(),
            },
            "dir" => {
                let dir_name = line[1].as_str();
                if get_node(Rc::clone(&current), dir_name).is_some() {
                    continue;
                } else {
                    let new_node = create_ref_node(0, dir_name, InodeType::Dir);
                    insert_node(Rc::clone(&current), new_node);
                }
            }
            // This should be a digit
            _ => {
                let size = first_char.parse::<usize>().unwrap();
                let name = line[1].clone();
                let child = create_ref_node(size, &name, InodeType::File);
                insert_node(Rc::clone(&current), child);
            }
        }
    }
}

pub fn run() {
    let root = Rc::new(RefCell::new(Inode::new(0, "/", InodeType::Root)));
    parse_input(Rc::clone(&root));
    root.as_ref().borrow().print_dfs(0);

    // parse_input(&mut root);
}
