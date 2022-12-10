//! Solution to [AoC 2022 Day 7](https://adventofcode.com/2022/day/7)

use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
enum LsNode {
    Directory(PathBuf),
    File(PathBuf, usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    Ls(Vec<LsNode>),
    Cd(PathBuf),
    CdRoot,
    CdUp,
}

#[derive(Debug, Clone, Hash)]
enum FsNode {
    Directory {
        /// Names are stored as absolute paths
        name: PathBuf,
        children: Vec<FsNode>,
        size: usize,
    },
    File {
        /// Names are stored as absolute paths
        name: PathBuf,
        size: usize,
    },
}

impl FsNode {
    /// Create a directory node
    fn directory(name: PathBuf, cache: &HashMap<PathBuf, Vec<LsNode>>) -> Self {
        let mut children = Vec::new();
        let mut dir_size = 0;
        for ls_node in &cache[&name] {
            let mut name = name.clone();
            children.push(match ls_node {
                LsNode::Directory(child_name) => {
                    name.push(child_name);
                    let dir_node = Self::directory(name, cache);
                    dir_size += dir_node.size();
                    dir_node
                }
                LsNode::File(child_name, size) => {
                    name.push(child_name);
                    dir_size += *size;
                    Self::file(name, *size)
                }
            });
        }
        Self::Directory {
            name,
            children,
            size: dir_size,
        }
    }

    /// Create a file node
    fn file(name: PathBuf, size: usize) -> Self {
        Self::File { name, size }
    }

    /// Return size of node
    fn size(&self) -> usize {
        *match self {
            Self::Directory { size, .. } | Self::File { size, .. } => size,
        }
    }
}

#[derive(Debug, Clone)]
struct Filesystem {
    root: FsNode,
}

impl Filesystem {
    fn from_commands(terminal: &[Command]) -> Self {
        // 1. build up a map of directories to their contents
        let mut ls_cache: HashMap<PathBuf, Vec<LsNode>> = HashMap::new();
        assert_eq!(terminal[0], Command::CdRoot);
        let mut pwd = PathBuf::from("/");
        for line in &terminal[1..] {
            match line {
                Command::Ls(nodes) => {
                    ls_cache.insert(pwd.clone(), nodes.clone());
                }
                Command::Cd(path) => pwd.push(path),
                Command::CdRoot => pwd = PathBuf::from("/"),
                // Assuming that we can never cd to a parent directory that we haven't yet visited
                Command::CdUp => pwd = pwd.parent().expect("a parent dir to exist").to_path_buf(),
            };
        }

        // 2. Use map to build up a filesystem tree
        let children = vec![FsNode::directory(PathBuf::from("/"), &ls_cache)];
        let size = children.iter().map(FsNode::size).sum();

        Self {
            root: FsNode::Directory {
                name: PathBuf::from("/"),
                children,
                size,
            },
        }
    }
}

fn part1(fs: &Filesystem) -> usize {
    // Find sum of sizes of directories with total size <= 100000
    const THRESHOLD: usize = 100000;
    let mut total_size = 0;
    let FsNode::Directory{ref children, ..} = fs.root else {
        panic!("Root is not a directory");
    };

    // Traverse filesystem tree, arbitrarily using non-recursive BFS
    let mut queue = children.clone();
    while let Some(node) = queue.pop() {
        let FsNode::Directory{children, size, ..} = node else {
            continue;
        };
        if size <= THRESHOLD {
            total_size += size;
        }
        queue.extend(children);
    }
    total_size
}

fn part2(fs: &Filesystem) -> usize {
    const FS_SPACE: usize = 70000000;
    const NEEDED_FREE_SPACE: usize = 30000000;

    let FsNode::Directory{ref children, size: used_space, ..} = fs.root else {
        panic!("Root is not a directory");
    };
    let free_space = FS_SPACE - used_space;
    assert!(free_space < NEEDED_FREE_SPACE, "What's the problem?");
    let space_to_free = NEEDED_FREE_SPACE - free_space;

    let mut closest: Option<usize> = None;

    // Traverse filesystem tree, arbitrarily using non-recursive BFS
    let mut queue = children.clone();
    while let Some(node) = queue.pop() {
        let FsNode::Directory{children, size, ..} = node else {
            continue;
        };
        // No need to traverse into directories that are too small
        if size < space_to_free {
            continue;
        }
        match closest {
            Some(prev_closest) if prev_closest < size => (),
            _ => closest = Some(size),
        };
        queue.extend(children);
    }
    closest.unwrap()
}

fn parse_commands(input: &str) -> Vec<Command> {
    let mut out = Vec::new();
    let mut lines = input.trim().lines().peekable();
    while lines.peek().is_some() {
        let cmd = lines
            .next()
            .unwrap()
            .strip_prefix("$ ")
            .expect("a $-prefixed line");
        out.push(if cmd == "ls" {
            let mut contents = Vec::new();
            loop {
                let line = match lines.peek() {
                    Some(line) if !line.starts_with('$') => lines.next().unwrap(),
                    _ => break,
                };
                contents.push(if line.starts_with("dir") {
                    LsNode::Directory(PathBuf::from(line.strip_prefix("dir ").unwrap()))
                } else {
                    let (size, name) = line.split_once(' ').expect("a file");
                    LsNode::File(PathBuf::from(name), size.parse().unwrap())
                });
            }
            Command::Ls(contents)
        } else if cmd == "cd /" {
            Command::CdRoot
        } else if cmd == "cd .." {
            Command::CdUp
        } else if let Some(cmd) = cmd.strip_prefix("cd ") {
            Command::Cd(PathBuf::from(cmd))
        } else {
            panic!("Invalid command");
        });
    }
    out
}

fn parse_input(input: &str) -> Filesystem {
    let commands = parse_commands(input);
    Filesystem::from_commands(&commands)
}

fn main() {
    let input = include_str!("../inputs/day7.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(95437, part1(&input));
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(24933642, part2(&input));
    }
}
