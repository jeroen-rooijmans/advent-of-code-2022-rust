# Day 3: Rucksack Reorganization

Due to a slight packing error, a few items needs to be rearranged in the rucksacks.
Each rucksack has two large **compartments**. All items of a given type are meant to go into the same compartment.
The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.

THe list of items for each rucksack is given as characters all on a single line. A given rucksack always has the same number of items
in each of the two compartments, so the first half of the characters represent items in the first compartment, and the second half
represents items in the second compartment.

For example, you have the following list of contents from six rucksacks:
```
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
```
* The first rucksack contains the items `vJrwpWtwJgWrhcsFMMfFFhFp`, which means its first compartment contains the items `vJrwpWtwJgWr`,
while the second compartment contains the items `hcsFMMfFFhFp`. The only item type that appears in both compartments is **`p`**.
* The second rucksack's compartments contain `jqHRNqRjqzjGDLGL` and `rsFMfFZSrLrFZsSL`. The only item type that appears in both compartments is **`L`**.
* The third rucksack's compartments contain `PmmdzqPrV` and `vPwwTWBwg`; the only common item type is **`P`**.
* The fourth rucksack's compartments only share item type **`v`**.
* The fifth rucksack's compartments only share item type **`t`**.
* The sixth rucksack's compartments only share item type **`s`**.

To help prioritize item rearrangement, every item type can be converted to a priority:
* Lowercase item typas `a` through `z` have priorities 1 through 26.
* Uppercase item types `A` through `Z` have priorities 27 through 52.

In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is **157**.

In order to solve part one of the puzzle, we need to compute the sum of priorities of the item types that need to be rearranged to fix the packing errors made by the silly Elf.

# Solution