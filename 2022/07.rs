use std::path::{Path, PathBuf};

use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Command<'_>> {
    input
        .split("$ ")
        .filter_map(|s| match s.is_empty() {
            true => None,
            false => Some(s.trim()),
        })
        .map(|raw| {
            let mut iter = raw.lines();
            let cmd = iter.next().unwrap();
            if cmd == "ls" {
                let size = iter
                    .filter_map(|f| {
                        f.split(' ').next().and_then(|size| match size {
                            "dir" => None,
                            s => Some(s.parse::<usize>().unwrap()),
                        })
                    })
                    .sum();
                Command::ListDir { size }
            } else if cmd.starts_with("cd") {
                let dir = cmd.split(' ').nth(1).unwrap();
                Command::ChangeDir { dir }
            } else {
                panic!("unknown command `{}`", cmd);
            }
        })
        .collect()
}

fn default_input() -> Vec<Command<'static>> {
    parse_input(include_str!("input/07.txt"))
}

#[derive(Debug, Clone)]
enum Command<'a> {
    ChangeDir { dir: &'a str },
    ListDir { size: usize },
}

// Traverse the commands and map what we know about the file system. There is no
// need to store the files so for each file we just update the size of the
// current working directory and all its parent directories.
fn build_fs(cmds: Vec<Command<'_>>) -> HashMap<PathBuf, usize> {
    let mut fs = HashMap::new();
    let mut cwd = PathBuf::from("/");
    for cmd in cmds {
        match cmd {
            Command::ChangeDir { dir } => {
                if dir == ".." {
                    cwd.pop();
                } else {
                    cwd.push(dir);
                }
            }
            Command::ListDir { size } => {
                for p in cwd.ancestors() {
                    *fs.entry(p.to_owned()).or_default() += size;
                }
            }
        }
    }
    fs
}

fn part1(cmds: Vec<Command<'_>>) -> usize {
    build_fs(cmds).into_values().filter(|&s| s <= 100_000).sum()
}

fn part2(cmds: Vec<Command<'_>>) -> usize {
    let fs = build_fs(cmds);
    let used = fs.get(Path::new("/")).unwrap();
    let delete = 30_000_000 - (70_000_000 - used);
    fs.into_values().sorted().find(|&s| s >= delete).unwrap()
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example() {
    let input = parse_input(
        "$ cd /
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
7214296 k",
    );

    assert_eq!(part1(input.clone()), 95437);
    assert_eq!(part2(input), 24933642);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1583951);
    assert_eq!(part2(input), 214171);
}
