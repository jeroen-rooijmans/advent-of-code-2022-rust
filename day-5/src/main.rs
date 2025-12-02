// Advent of Code 2022 - Day 5

const INPUT: &str = include_str!("./input.txt");

/// Represents a ship that contains a certain number of crate stacks.
/// Each create is represented by the char given in the input.
#[derive(Debug)]
struct Ship {
    stacks: Vec<Vec<char>>,
}

impl Ship {
    fn init(starting_stacks: &str) -> Self {
        // Create empty ship with appropiate number of stacks.
        let mut ship = Ship {
            stacks: vec![vec![]; get_number_of_stacks(starting_stacks)],
        };
        // Fill ship according to starting stacks
        starting_stacks.lines().rev().skip(1).for_each(|l| {
            for it in l.chars().skip(1).step_by(4).enumerate() {
                if it.1.is_alphabetic() {
                    ship.push(it.0, it.1);
                }
            }
        });
        ship
    }

    /// Compiles message from crates at the top of each stack
    fn get_message(&self) -> String {
        self.stacks
            .iter()
            .filter_map(|stack| stack.last())
            .collect::<String>()
    }

    /// Pushes a crate on top of the stack identified by idx.
    fn push(&mut self, idx: usize, ch: char) {
        self.stacks[idx].push(ch);
    }

    /// Removes a crate from the top of the stack identified by idx.
    fn pop(&mut self, idx: usize) -> char {
        self.stacks[idx]
            .pop()
            .expect("Stack should contain at least one item.")
    }

    /// Moves multiple crates one by one from one stack to another.
    /// Note: given idices are adjusted to 0-based indexing.
    fn move_crates(&mut self, n: usize, from_idx: usize, to_idx: usize) {
        for _ in 0..n {
            let moving_crate = self.pop(from_idx - 1);
            self.push(to_idx - 1, moving_crate);
        }
    }

    /// Moves multiple crates as a stack in one operation.
    /// Note: given indices are adjusted to 0-based indexing.
    fn move_stack_of_crates(&mut self, n: usize, from_idx: usize, to_idx: usize) {
        let stack_crates = &mut self.stacks[from_idx - 1];
        let mut moving_stack = stack_crates
            .drain(stack_crates.len() - n..)
            .collect::<Vec<char>>();
        self.stacks[to_idx - 1].append(&mut moving_stack);
    }
}

fn get_number_of_stacks(starting_stacks: &str) -> usize {
    starting_stacks
        .lines()
        .last()
        .expect("Starting stack should contain multiple lines.")
        .split_whitespace()
        .last()
        .and_then(|s| s.parse::<usize>().ok())
        .expect("Last number in starting stack should be parsable.")
}

fn parse_rearrangement_procedure(procedure: &str) -> Vec<(usize, usize, usize)> {
    procedure
        .lines()
        .map(|l| {
            let mut iter = l.split_whitespace().filter_map(|s| s.parse().ok());
            let from = iter
                .next()
                .expect("Should contain the numbers of crates to move.");
            let from_position = iter.next().expect("Should contain the 'from' crate index.");
            let to = iter.next().expect("Should contain the 'to' crate index.");
            (from, from_position, to)
        })
        .collect()
}

fn solve_part_one(starting_stacks: &str, procedure: &str) -> String {
    // Fill ship according to starting_stacks
    let mut ship = Ship::init(starting_stacks);
    // Parse procedure and apply the steps moving a single crate at the time.
    let procedure = parse_rearrangement_procedure(procedure);
    for proc in procedure {
        ship.move_crates(proc.0, proc.1, proc.2);
    }
    // Find the crates that ended at the top of each stack.
    ship.get_message()
}

fn solve_part_two(starting_stacks: &str, procedure: &str) -> String {
    // Fill ship according to starting_stacks
    let mut ship = Ship::init(starting_stacks);
    // Parse procedure and apply the steps moving stacks of crates at the time.
    let procedure = parse_rearrangement_procedure(procedure);
    for proc in procedure {
        ship.move_stack_of_crates(proc.0, proc.1, proc.2);
    }
    // Find the crates that ended at the top of each stack.
    ship.get_message()
}

fn main() {
    let (starting_stacks, procedure) = INPUT
        .split_once("\n\n")
        .unwrap_or_else(|| panic!("Error while parsing input!"));
    let answer_part_one = solve_part_one(starting_stacks, procedure);
    println!("Part one: the top crates create the message: {answer_part_one}");
    let answer_part_two = solve_part_two(starting_stacks, procedure);
    println!("Part two: the top crates create the message: {answer_part_two}");
}
