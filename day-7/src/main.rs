// Advent of Code 2022 - Day 7: No Space Left On Device

use std::collections::HashMap;

const INPUT: &str = include_str!("./input.txt");

struct Filesystem {
    root: Directory,
    current_path: Vec<String>,
}

impl Filesystem {
    fn new() -> Self {
        Self {
            root: Directory::new("/"),
            current_path: Vec::new(),
        }
    }

    /// Returns mutable reference to current directory in the `Filesystem` based on `current_path`.
    fn current_dir_mut(&mut self) -> &mut Directory {
        let mut dir = &mut self.root;
        for name in &self.current_path {
            dir = dir.get_or_create_dir_mut(name);
        }
        dir
    }

    /// Handle a 'cd' command, updating the current path.
    fn cd(&mut self, arg: &str) {
        match arg {
            "/" => self.current_path.clear(),
            ".." => {
                self.current_path.pop();
            }
            name => {
                self.current_dir_mut().get_or_create_dir_mut(name);
                self.current_path.push(name.to_string());
            }
        }
    }

    /// Add a file to the current directory.
    fn add_file(&mut self, name: &str, size: u64) {
        self.current_dir_mut().files.insert(name.to_string(), size);
    }
}

#[derive(Debug)]
struct Directory {
    #[allow(unused)]
    name: String,
    files: HashMap<String, u64>,
    directories: HashMap<String, Directory>,
}

impl Directory {
    fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            files: HashMap::new(),
            directories: HashMap::new(),
        }
    }

    fn size(&self) -> u64 {
        let file_sum: u64 = self.files.values().copied().sum();
        let dir_sum: u64 = self.directories.values().map(Directory::size).sum();
        file_sum + dir_sum
    }

    fn get_or_create_dir_mut(&mut self, name: &str) -> &mut Directory {
        self.directories
            .entry(name.to_string())
            .or_insert_with(|| Directory::new(name))
    }

    fn collect_dir_sizes(&self) -> Vec<u64> {
        let mut sizes = Vec::new();
        sizes.push(self.size());
        // Recursively call the function on all subdirectories
        // Extend the current vector with vectors returned from the recursive calls
        for dir in self.directories.values() {
            sizes.extend(dir.collect_dir_sizes());
        }
        sizes
    }
}

fn parse_input(input: &str) -> Directory {
    let mut fs = Filesystem::new();
    for line in input.trim().lines() {
        if let Some(arg) = line.strip_prefix("$ cd ") {
            fs.cd(arg);
        } else if line.starts_with("$ ls") || line.starts_with("dir ") {
        } else {
            // file: "<size> <name>"
            let mut parts = line.split_whitespace();
            let size: u64 = parts.next().unwrap().parse().unwrap();
            let name: &str = parts.next().unwrap();
            fs.add_file(name, size);
        }
    }
    fs.root
}

fn solve_part_one(input: &str) -> u64 {
    let fs = parse_input(input);
    let dir_sizes = fs.collect_dir_sizes();
    dir_sizes.into_iter().filter(|&size| size <= 100_000).sum()
}

fn solve_part_two(input: &str) -> u64 {
    let fs = parse_input(input);
    let used_space = fs.size();
    let dir_sizes = fs.collect_dir_sizes();
    let current_free_disk_space = 70_000_000 - used_space;
    let target_size = 30_000_000 - current_free_disk_space;
    dir_sizes
        .into_iter()
        .filter(|&size| size >= target_size)
        .min()
        .unwrap()
}

fn main() {
    let part_one_answer = solve_part_one(INPUT);
    println!("Part one:\n{part_one_answer}");
    let part_two_answer = solve_part_two(INPUT);
    println!("Part two:\n{part_two_answer}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = "$ cd /
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
7214296 k";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 95437);
    }

    #[test]
    fn part2() {
        let example_input = "$ cd /
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
7214296 k";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 24933642);
    }
}
