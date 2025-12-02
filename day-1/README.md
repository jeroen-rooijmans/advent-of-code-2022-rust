# Day 1: Calorie Counting

The Elves have written down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them. One item per line. Each Elf seperates their own inventory from the previous Elf's inventory (if any) by a blank line.

For example:
```
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
```
This list represents the Calories of the food carried by five Elves:
* The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
* The second Elf is carrying one food item with 4000 Calories.
* The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
* The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
* The fifth Elf is carrying one food item with 10000 Calories.

For part one, the Elves like to know how many Calories are being carried by the Elf carrying the **most** Calories. In the example above, this is `24000` (carried by the fourth Elf).
For part two, the Elves want to known the total of Calories carried by the **top three** Elves carrying the most Calories. In the example above, this `45000` (the fourth Elf with `24000`, the third Elf with `11000` and the fifth Elf with `10000`).

## Solution

Both parts could quickly be solved after parsing the input as follows:
* Read input as &str
* Split input string slice on blank lines ("\n\n")
* Compute total Calories for each inventory by iterating over resulting string slices from the previous step, parsing each line to a number of type `usize`, and summing the parsed numbers for each inventory.
* Sort the resulting vector in descending order

The highest total of Calories for each inventory is now at the first element in the vector, this number solves part one of the puzzle. In order to solve part two we simply slice the first 3 elements from the vector and sum these to obtain the total number of Calories carried by the top 3 Elves.