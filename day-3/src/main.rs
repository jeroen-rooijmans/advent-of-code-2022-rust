// Advent of Code 2022 - Day 3

/*
The puzzle specifies that there are 52 different possible items.
We can represent the unique set of these items using a 'u64', setting bits in order of increasing priority.
Starting with 'a' at 2^1, the number of trailing zeros will be equal to priority.
A few examples:
'a' in bits: (2^1 = 2)
0b0000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000010
                                                                  'a'-^
'b' in bits: (2^2 = 4)
0b0000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000100
                                                                 'b'-^
'A' in bits: (2^27 = 134_217_728)
0b0000_0000000000_0000000000_0000000000_0010000000_0000000000_0000000000
                                      'A'-^
'Z' in bits: (2^52 = 4_503_599_627_370_496)
0b0000_0000000100_0000000000_0000000000_0000000000_0000000000_0000000000
          'Z'-^
'a', 'z', 'A' and 'Z' in bits: (2^52 + 2^27 + 2^26 + 2^1 = 4_503_599_828_697_090)
0b0000_0000000100_0000000000_0000000000_0011000000_0000000000_0000000010
          'Z'-^                       'A'-^^-'z'                  'a'-^
NOTE: this representation ignores multiple items of the same kind in a single compartment!
*/

const INPUT: &str = include_str!("./input.txt");

// ASCII 'a' corresponds to 97, item 'a' has priority 1
// ASCII 'A' corresponds to 65, item 'A' has priority 27
const LOWERCASE_OFFSET: u32 = 'a' as u32 - 1; // 97 - 1 = 96
const UPPERCASE_OFFSET: u32 = 'A' as u32 - 27; // 65 - 27 = 38

/// An `Item` represents items carried in a `Rucksack`.
#[derive(Debug)]
struct Item(u64);
impl Item {
    /// Calculate the priority for an `Item`, which is equal to the number of trailing zeros in our representation.
    fn priority(&self) -> u32 {
        self.0.trailing_zeros()
    }
}

impl From<char> for Item {
    /// Convert alphabetic char to `Item`
    fn from(value: char) -> Self {
        assert!(
            value.is_alphabetic(),
            "Encountered non-alphabetic char: {value}, cannot convert to an Item!"
        );
        /* For convenience, we can make sure the number of trailing zeros in the binary representation equal to the priority value for an `Item`.
         * In order to compute the priority value, an appropiate offset is subtracted from their ASCII value. Uppercase alphabetic and lowercase
         * alphabetic ASCII characters both are represented by an incrementing range of values, which respectively start with 'A' and 'a'.
         * By computing the ASCII value of these characters and subtracting their priority value, we can easily compute the offset for all lowercase
         * and uppercase characters. Finally we shift an 1 bit left a number of times, given by the priority value and create an `Item` with this value.
         */
        let offset = if value > 'Z' {
            LOWERCASE_OFFSET
        } else {
            UPPERCASE_OFFSET
        };
        let priority = value as u32 - offset;
        let set_bit = 1 << priority;
        Item(set_bit)
    }
}

impl From<Compartment> for Item {
    /// Convert `Compartment` with a single item to `Item`
    fn from(value: Compartment) -> Self {
        assert!(
            value.0.is_power_of_two(),
            "{value:?} contains no items or more than one item!"
        );
        Item(value.0)
    }
}

/// A `Compartment` represents the unique items held in each compartment of a `Rucksack`,
/// by assigning each type of `Item` to a particular bit in the inner `u64`.
#[derive(Debug, Default)]
struct Compartment(u64);

impl Compartment {
    /// Insert `Item` into `Compartment` by applying bitwise OR
    fn insert(&mut self, item: &Item) {
        self.0 |= item.0;
    }

    /// Create a `Compartment` that only contains items that are in both `Compartment`s 'self' and 'other' by taking an intersectiong using biswise AND.
    fn intersect(&self, other: &Compartment) -> Self {
        Compartment(self.0 & other.0)
    }
}

impl From<&str> for Compartment {
    /// Create a `Compartment` filled with items represented by string slice
    fn from(value: &str) -> Self {
        // create empty compartment
        let mut compartment = Compartment::default();
        // convert each `char` into `Item` and insert into the compartment
        for c in value.chars() {
            let item = Item::from(c);
            compartment.insert(&item);
        }
        compartment
    }
}

/// A `Rucksack` contains `Item`s distributed over its two `Compartment`s
#[derive(Debug)]
struct Rucksack(Compartment, Compartment);

impl Rucksack {
    /// Identify commen `Item` in both compartments
    fn find_common_item(&self) -> Item {
        self.0.intersect(&self.1).into()
    }

    /// Combine all unique items from both compartments into a new `Compartment`
    fn combine_into_compartment(&self) -> Compartment {
        let (compartment1, compartment2) = (self.0 .0, self.1 .0);
        Compartment(compartment1 | compartment2)
    }
}

impl From<&str> for Rucksack {
    /// Create a `Rucksack` with its `Compartment`s filled with `Item`s represented by the string slice.
    // The two halves of the characters represents `Item`s in the seperate `Compartment`s
    fn from(value: &str) -> Self {
        let (str1, str2) = value.split_at(value.len() / 2);
        assert!(
            str1.len() == str2.len(),
            "Invalid line length {value:?} ({:?}), cannot split into two equal Compartments!",
            value.len()
        );
        let compartment1 = Compartment::from(str1);
        let compartment2 = Compartment::from(str2);
        Rucksack(compartment1, compartment2)
    }
}

fn solve_part_one(input: &[Rucksack]) -> u32 {
    // find the priority value of the common item in each rucksack and return the sum for all rucksacks.
    input
        .iter()
        .map(Rucksack::find_common_item)
        .map(|i| i.priority())
        .sum::<u32>()
}

fn solve_part_two(input: &[Rucksack]) -> u32 {
    // find the priority value of the common item in each group of 3 rucksacks and return the sum for all groups.
    input
        .iter()
        .map(Rucksack::combine_into_compartment)
        .collect::<Vec<Compartment>>()
        .chunks(3)
        .map(|group| group[0].intersect(&group[1]).intersect(&group[2]))
        .map(Item::from)
        .map(|i| i.priority())
        .sum()
}

fn main() {
    // parse input as vector of `Rucksack`s
    let input: Vec<Rucksack> = INPUT.lines().map(Rucksack::from).collect();
    dbg!(&input);
    let part_one_answer = solve_part_one(&input);
    println!(
        "Total sum of priorities for items that appear in both compartments: {part_one_answer:?}"
    );
    let part_two_answer = solve_part_two(&input);
    println!(
        "Total sum of priorities for items that correspond to the badges: {part_two_answer:?}"
    );
}
