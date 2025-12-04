// Advent of Code 2022 - Day 8: Treetop Tree House

const INPUT: &str = include_str!("./input.txt");

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    // tree heights are increased by 1 to simplify comparison with u32 type
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() + 1).collect())
        .collect()
}

fn viewing_distance(grid: &[Vec<u32>], row: usize, col: usize, direction: Direction) -> usize {
    let tree_height = grid[row][col];
    let mut distance = 0;
    match direction {
        Direction::Up => {
            for r in (0..row).rev() {
                distance += 1;
                if grid[r][col] >= tree_height {
                    break;
                }
            }
        }
        Direction::Down => {
            for r in (row + 1)..grid.len() {
                distance += 1;
                if grid[r][col] >= tree_height {
                    break;
                }
            }
        }
        Direction::Left => {
            for c in (0..col).rev() {
                distance += 1;
                if grid[row][c] >= tree_height {
                    break;
                }
            }
        }
        Direction::Right => {
            for c in (col + 1)..grid[0].len() {
                distance += 1;
                if grid[row][c] >= tree_height {
                    break;
                }
            }
        }
    }
    distance
}

fn solve_part_one(input: &str) -> usize {
    let grid = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut is_visible = vec![vec![false; cols]; rows];

    for r in 0..rows {
        // visible from left check
        let mut max_height = 0;
        for c in 0..cols {
            let height = grid[r][c];
            if height > max_height {
                is_visible[r][c] = true;
                max_height = height;
            }
        }
        // visible from right check
        let mut max_height = 0;
        for c in (0..cols).rev() {
            let height = grid[r][c];
            if height > max_height {
                is_visible[r][c] = true;
                max_height = height;
            }
        }
    }

    for c in 0..cols {
        // visible from top check
        let mut max_height = 0;
        for r in 0..rows {
            let height = grid[r][c];
            if height > max_height {
                is_visible[r][c] = true;
                max_height = height;
            }
        }
        // visible from bottom check
        let mut max_height = 0;
        for r in (0..rows).rev() {
            let height = grid[r][c];
            if height > max_height {
                is_visible[r][c] = true;
                max_height = height;
            }
        }
    }
    // count total number of visible trees
    is_visible.iter().flatten().filter(|&&v| v).count()
}

fn solve_part_two(input: &str) -> usize {
    let grid = parse_input(input);
    let mut max_scenic_score = 0;

    for r in 1..(grid.len() - 1) {
        for c in 1..(grid[0].len() - 1) {
            let current_scenic_score = viewing_distance(&grid, r, c, Direction::Up)
                * viewing_distance(&grid, r, c, Direction::Down)
                * viewing_distance(&grid, r, c, Direction::Left)
                * viewing_distance(&grid, r, c, Direction::Right);
            if current_scenic_score > max_scenic_score {
                max_scenic_score = current_scenic_score;
            }
        }
    }
    max_scenic_score
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
        let example_input = "30373
25512
65332
33549
35390";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 21);
    }

    #[test]
    fn part2() {
        let example_input = "30373
25512
65332
33549
35390";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 8);
    }
}
