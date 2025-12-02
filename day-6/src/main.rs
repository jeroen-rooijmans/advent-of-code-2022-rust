// Advent of Code 2022 - Day 6: Tuning Trouble

const INPUT: &str = include_str!("./input.txt");

fn parse_input(input: &str) -> &[u8] {
    input.trim().as_bytes()
}

fn all_unique<const N: usize>(bytes: &[u8; N]) -> bool {
    for i in 0..N {
        for j in i + 1..N {
            if bytes[i] == bytes[j] {
                return false;
            }
        }
    }
    true
}

fn solve_part_one(input: &str) -> usize {
    let buffer = parse_input(input);
    for i in 0..=(buffer.len() - 4) {
        let window: &[u8; 4] = buffer[i..i + 4].try_into().unwrap();
        if all_unique(window) {
            return i + 4;
        }
    }
    panic!("Input should contain a `start-of-packet` marker");
}

fn solve_part_two(input: &str) -> usize {
    let buffer = parse_input(input);
    for i in 0..=(buffer.len() - 14) {
        let window: &[u8; 14] = buffer[i..i + 14].try_into().unwrap();
        if all_unique(window) {
            return i + 14;
        }
    }
    panic!("Input should contain a `start-of-message` marker");
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
    fn part1_example1() {
        let example_input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 7);
    }

    #[test]
    fn part1_example2() {
        let example_input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 5);
    }

    #[test]
    fn part1_example3() {
        let example_input = "nppdvjthqldpwncqszvftbrmjlhg";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 6);
    }

    #[test]
    fn part1_example4() {
        let example_input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 10);
    }

    #[test]
    fn part1_example5() {
        let example_input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 11);
    }

    #[test]
    fn part2_example1() {
        let example_input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 19);
    }

    #[test]
    fn part2_example2() {
        let example_input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 23);
    }

    #[test]
    fn part2_example3() {
        let example_input = "nppdvjthqldpwncqszvftbrmjlhg";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 23);
    }

    #[test]
    fn part2_example4() {
        let example_input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 29);
    }

    #[test]
    fn part2_example5() {
        let example_input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 26);
    }
}
