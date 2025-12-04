// Advent of Code 2022 - Day 9: Rope Bridge
use std::collections::HashSet;

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "U" => Some(Direction::Up),
            "D" => Some(Direction::Down),
            "L" => Some(Direction::Left),
            "R" => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Motion {
    direction: Direction,
    steps: u8,
}

fn parse_input(input: &str) -> Vec<Motion> {
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let direction = Direction::from_str(parts.next()?)?;
            let steps = parts.next()?.parse::<u8>().ok()?;
            Some(Motion { direction, steps })
        })
        .collect()
}

fn move_head(head: &mut Coordinate, direction: &Direction) {
    match direction {
        Direction::Up => head.y += 1,
        Direction::Right => head.x += 1,
        Direction::Down => head.y -= 1,
        Direction::Left => head.x -= 1,
    }
}

fn move_tail(tail: &mut Coordinate, head: &Coordinate) {
    if (head.x - tail.x).abs() > 1 || (head.y - tail.y).abs() > 1 {
        tail.x += (head.x - tail.x).signum();
        tail.y += (head.y - tail.y).signum();
    }
}

fn solve_part_one(input: &str) -> usize {
    let motions = parse_input(input);
    let mut head = Coordinate { x: 0, y: 0 };
    let mut tail = Coordinate { x: 0, y: 0 };
    let mut visited_positions: HashSet<Coordinate> = HashSet::from([tail]);

    for motion in motions {
        for _ in 0..motion.steps {
            move_head(&mut head, &motion.direction);
            move_tail(&mut tail, &head);
            visited_positions.insert(tail);
        }
    }
    visited_positions.len()
}

fn solve_part_two(input: &str) -> usize {
    let motions = parse_input(input);
    let mut rope = vec![Coordinate { x: 0, y: 0 }; 10];
    let mut visited_positions: HashSet<Coordinate> = HashSet::from([*rope.last().unwrap()]);

    for motion in motions {
        for _ in 0..motion.steps {
            move_head(&mut rope[0], &motion.direction);
            for i in 1..rope.len() {
                let prev_knot = rope[i - 1];
                move_tail(&mut rope[i], &prev_knot);
            }
            visited_positions.insert(*rope.last().unwrap());
        }
    }
    visited_positions.len()
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
        let example_input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 13);
    }

    #[test]
    fn part2_example1() {
        let example_input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 1);
    }

    #[test]
    fn part2_example2() {
        let example_input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 36);
    }
}
