// Advent of Code 2022 - Day 2

use nom::{
    Parser,
    character::complete::{anychar, space1},
    error::{Error, ErrorKind},
    sequence::separated_pair,
};

const INPUT: &str = include_str!("./input.txt");

/// Represents the strategy guide as a vector of character pairs
type Input = Vec<(char, char)>;

// Attempt to parse a line from the INPUT
fn parse_line(line: &str) -> Result<(char, char), Error<&str>> {
    match separated_pair(anychar, space1, anychar).parse(line) {
        Ok(("", pair)) => Ok(pair),
        Ok((remaining, _)) => Err(Error::new(remaining, ErrorKind::NonEmpty)),
        Err(nom::Err::Error(e) | nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => Err(Error::new(line, ErrorKind::Eof)),
    }
}

/// Represents a shape in a game of Rock, Paper, Scissors
#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

/// Convert a Shape into its score value
impl From<Shape> for usize {
    fn from(shape: Shape) -> Self {
        match shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

/// Trait for converting into a Shape.
trait TryIntoShape {
    type Error;
    fn try_into_shape(&self) -> Result<Shape, Self::Error>;
}

/// Implementation for converting characters from the encrypted strategy guide into a Shape
impl TryIntoShape for char {
    type Error = &'static str;

    /// Attempt to convert character into Shape
    fn try_into_shape(&self) -> Result<Shape, Self::Error> {
        match self {
            'A' | 'X' => Ok(Shape::Rock),
            'B' | 'Y' => Ok(Shape::Paper),
            'C' | 'Z' => Ok(Shape::Scissors),
            _ => Err("Character cannot be converted into Shape!"),
        }
    }
}

/// Represents an outcome of a round of Rock, Paper, Sciccors
enum Outcome {
    Win(Shape),
    Draw(Shape),
    Loss(Shape),
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            // 6 points for win plus points for your shape
            Outcome::Win(s) => 6 + usize::from(*s),
            // 3 points for draw plus points for your shape
            Outcome::Draw(s) => 3 + usize::from(*s),
            // 0 points for loss plus points for your shape
            Outcome::Loss(s) => usize::from(*s),
        }
    }
}

/// Trait for converting into an Outcome
trait TryIntoOutcome {
    type Error;
    fn try_into_outcome(&self) -> Result<Outcome, Self::Error>;
}

/// Implementation for converting a pair of Shapes into an Outcome
impl TryIntoOutcome for (Shape, Shape) {
    type Error = &'static str;

    fn try_into_outcome(&self) -> Result<Outcome, Self::Error> {
        let (elf, player) = self;

        let outcome = match (elf, player) {
            (Shape::Rock, Shape::Rock)
            | (Shape::Paper, Shape::Paper)
            | (Shape::Scissors, Shape::Scissors) => Outcome::Draw(*player),
            (Shape::Rock, Shape::Paper)
            | (Shape::Paper, Shape::Scissors)
            | (Shape::Scissors, Shape::Rock) => Outcome::Win(*player),
            (Shape::Rock, Shape::Scissors)
            | (Shape::Paper, Shape::Rock)
            | (Shape::Scissors, Shape::Paper) => Outcome::Loss(*player),
        };
        Ok(outcome)
    }
}

/// Implementation for converting a pair of chars into an Outcome
impl TryIntoOutcome for (char, char) {
    type Error = &'static str;

    fn try_into_outcome(&self) -> Result<Outcome, Self::Error> {
        let (ch1, ch2) = self;
        let elf = ch1.try_into_shape().unwrap();

        match (elf, ch2) {
            (Shape::Rock, 'X') => Ok(Outcome::Loss(Shape::Scissors)),
            (Shape::Paper, 'X') => Ok(Outcome::Loss(Shape::Rock)),
            (Shape::Scissors, 'X') => Ok(Outcome::Loss(Shape::Paper)),
            (_, 'Y') => Ok(Outcome::Draw(elf)),
            (Shape::Rock, 'Z') => Ok(Outcome::Win(Shape::Paper)),
            (Shape::Paper, 'Z') => Ok(Outcome::Win(Shape::Scissors)),
            (Shape::Scissors, 'Z') => Ok(Outcome::Win(Shape::Rock)),
            _ => Err("Characters cannot be converted into Outcome!"),
        }
    }
}

fn main() {
    // parse input as vector with tuples of chars
    let input: Input = INPUT.lines().flat_map(parse_line).collect();

    // Part one
    // decrypt the chars according to our intuition into Rock, Paper, or Sciccors shape
    let shapes: Vec<_> = input
        .iter()
        .map(|t| {
            let (ch1, ch2) = t;
            (ch1.try_into_shape().unwrap(), ch2.try_into_shape().unwrap())
        })
        .collect();

    // determine outcome for each round
    let outcomes: Vec<_> = shapes
        .iter()
        .map(|s| s.try_into_outcome().unwrap())
        .collect();
    // compute sum of scores for each outcome to determine total score for strategy guide
    let total_score: usize = outcomes.iter().map(Outcome::score).sum();
    println!(
        "Following my interpretation of the encrypted strategy guide, I'll score {total_score:?} points."
    );

    // Part two
    // decrypt the chars into the correct Rock, Paper, Sciccors shape
    let outcomes: Vec<_> = input
        .iter()
        .map(|t| t.try_into_outcome().unwrap())
        .collect();
    // compute sum of scores for each outcome to determine total score for correct interpretation of strategy guide
    let total_score: usize = outcomes.iter().map(Outcome::score).sum();
    println!(
        "Following the correct interpretation of the encrypted strategy guide, I'll score {total_score:?} points."
    );
}
