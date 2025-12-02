# Day 2: Rock Paper Scissors

One Elf decides to share an **encrypted strategy guide** for the Rock Paper Scissors tournament.

This guide shows two columns of characters. The Elf claims that the first column is what your opponent is going to play: `A` for Rock, `B` for Paper, and `Z` for Scissors. He fails to explain what the second column represents, but it could be what we should play in response: `X` for Rock, `Y` for Paper, `Z` for Scissors.

The winner of the whole tournament is the player with the highest score. Your **total score** is the sum of your scores for each round. The score for a single round is the score for the **shape** we played (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the **outcome** of the round (0 if we lost, 3 if the round was a draw, and 6 if we won).

For example, suppose you were given the following strategy guide:
```
A Y
B X
C Z
```
This strategy guide predicts and recommends the following:
* In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
* In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
* The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.

In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6). For part one, we want to calculate our final score if we follow our interpretation of the strategy guide.

For part two, we learn that the correct interpretation of the second column in the strategy guide is as follows: The second column says how the round needs to end: `X` means you need to lose, `Y` means you need to end the round in a draw, and `Z` means you need to win. The final score is calculated the same way, and for part two we'd also would like to known our final score.

## Solution
For starters, we parse the input using the `nom` crate. This allows us to write a fairly simple parse_line function that returns a `Result<(char, char)>` if successful. We apply this to all lines in the raw input and end up with a `Vec<(char, char)>`.

By defining a `Shape` enum with `Rock`, `Paper`, and `Scissors` as options, and implement the `TryIntoShape` trait for `char` with a match block, we can now further parse the input to a `Vec<(Shape, Shape)>`. We also implement the `From<Shape>` trait for `usize`, where we can convert a Shape into its score value.

Now we define an `Outcome` enum, that has a `score` function to compute the score for a round of Rock, Paper, Scissors based on the `Outcome` and played `Shape`.
Similarly to what we did before, we implement a `TryIntoOutcome` trait for `(Shape, Shape)`, this conversion is the interpretation of part one of the puzzle, using a match block to determine if the `Outcome` is a `Win`, `Draw`, or a `Loss`, the `Outcome` includes the `Shape` played by us, in order to score it later on.

For part two we implement the `TryIntoOutcome` trait for `(char, char)`, we convert the first `char` to a `Shape`, and match the second `char` to determine the correct `Outcome` and `Shape` we play.

Now we are able to create a `Vec<usize>` for both parts of the puzzle, that contains the score for each round according to the puzzle's specificiation, and get the answers we are looking for by summing the values in this `Vec`.
