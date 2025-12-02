// Advent of Code 2022 - Day 1

const INPUT: &str = include_str!("./input.txt");

fn main() {
    // parse input by summing numbers from each Elf's inventory seperately.
    let mut calorie_per_elf = INPUT
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|l| l.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<_>>();
    // sort inventory sums in descending order
    calorie_per_elf.sort_unstable_by(|a, b| b.cmp(a));
    // Part one
    let highest_total = calorie_per_elf[0];
    println!("The Elf carrying the most Calories is carrying a sum of {highest_total} Calories.");
    // Part two
    let top_3_total: usize = calorie_per_elf[0..3].iter().sum();
    println!("The top three Elves carrying the most Calories are carrying a sum of {top_3_total} Calories.");
}
