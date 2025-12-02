// Advent of Code 2022 - Day 4
use std::ops::Range;

const INPUT: &str = include_str!("./input.txt");

fn parse_into_range(input: &str) -> Range<u32> {
    let (start, stop) = input
        .split_once('-')
        .and_then(|(l, r)| Some((l.parse().ok()?, r.parse().ok()?)))
        .expect("Input should contain a single -");
    start..stop
}

trait Contains {
    ///Returns true if self contains other, otherwise return false
    fn contains(&self, other: &Self) -> bool;

    /// Default implementation for symmetric property of equality
    fn contains_or_contained(&self, other: &Self) -> bool {
        self.contains(other) || other.contains(self)
    }
}

impl Contains for Range<u32> {
    /// A Range fully contains another range if it start before or at the same value as the other,
    /// and stops after or at the same value as the other.
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

trait Overlaps {
    /// Returns true if self overlaps other, otherwise return false
    fn overlaps(&self, other: &Self) -> bool;
}

impl Overlaps for Range<u32> {
    /// A Range overlaps with another range if it start before or at the same value as the other, and stops after or at the same value as the other one.
    fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

fn solve_part_one(input: &[(Range<u32>, Range<u32>)]) -> usize {
    input
        .iter()
        .filter(|pair| pair.0.contains_or_contained(&pair.1))
        .count()
}

fn solve_part_two(input: &[(Range<u32>, Range<u32>)]) -> usize {
    input.iter().filter(|pair| pair.0.overlaps(&pair.1)).count()
}

fn main() {
    let input: Vec<(Range<u32>, Range<u32>)> = INPUT
        .lines()
        .map(|l| l.split_once(',').expect("Lines should contain a single ,"))
        .map(|(l, r)| (parse_into_range(l), parse_into_range(r)))
        .collect();
    dbg!(&input);
    let answer_part_one = solve_part_one(&input);
    println!("Total number of assignment pairs where one range fully contain the other: {answer_part_one}");
    let answer_part_two = solve_part_two(&input);
    println!("Total number of assignment pairs where ranges overlap: {answer_part_two}");
}
