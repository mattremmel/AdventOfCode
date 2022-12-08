use crate::common::Challenge;
use itertools::Itertools;
use std::fmt::Debug;
use std::str::FromStr;

/**
--- Day 2: Rock Paper Scissors ---
The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack
storage, a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round,
the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a
winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats
Rock. If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle
input) that they say will be sure to help you win. "The first column is what your opponent is going
to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is
called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper,
and Z for Scissors. Winning every time would be suspicious, so the responses must have been
carefully chosen.

The winner of the whole tournament is the player with the highest score. Your total score is the sum
of your scores for each round. The score for a single round is the score for the shape you selected
(1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you
lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the
score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:

```
A Y
B X
C Z
```

This strategy guide predicts and recommends the following:

In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends
in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends
in a loss for you with a score of 1 (1 + 0).
The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
In this example, if you were to follow the strategy guide, you would get a total score
of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly according to your strategy guide?

Answer: 9241

--- Part Two ---
The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says
how the round needs to end: X means you need to lose, Y means you need to end the round in a draw,
and Z means you need to win. Good luck!"

The total score is still calculated in the same way, but now you need to figure out what shape to
choose so the round ends as indicated. The example above now goes like this:

In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y),
so you also choose Rock. This gives you a score of 1 + 3 = 4.
In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a
score of 1 + 0 = 1.
In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total
score of 12.

Following the Elf's instructions for the second column, what would your total score be if everything
goes exactly according to your strategy guide?

Answer: 14610
*/
pub struct Day2;
type Score = u32;

impl Challenge for Day2 {
    type Ret = Score;

    fn part1<T>(input: T) -> Self::Ret
    where
        T: AsRef<str>,
    {
        Day2::round_iter::<Move>(input.as_ref())
            .map(|(theirs, mine)| mine.score(&theirs))
            .sum::<Score>()
    }

    fn part2<T>(input: T) -> Self::Ret
    where
        T: AsRef<str>,
    {
        Day2::round_iter::<GameResult>(input.as_ref())
            .map(|(theirs, result)| result.score(&theirs))
            .sum::<Score>()
    }
}

impl Day2 {
    fn round_iter<RHS>(input: &str) -> impl Iterator<Item = (Move, RHS)> + '_
    where
        RHS: FromStr,
        <RHS as FromStr>::Err: Debug,
    {
        input.lines().map(|line| {
            line.split(' ')
                .next_tuple::<(_, _)>()
                .map(|(lhs, rhs)| (lhs.parse::<Move>().unwrap(), rhs.parse::<RHS>().unwrap()))
                .unwrap()
        })
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Copy, Clone, PartialEq)]
enum GameResult {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl Move {
    fn play(&self, other: &Move) -> GameResult {
        if self == other {
            return GameResult::Draw;
        }

        match (self, other) {
            (Move::Rock, Move::Scissors)
            | (Move::Scissors, Move::Paper)
            | (Move::Paper, Move::Rock) => GameResult::Win,
            _ => GameResult::Lose,
        }
    }

    fn score(&self, other: &Move) -> Score {
        *self as Score + self.play(other) as Score
    }
}
impl GameResult {
    fn move_for(&self, other: &Move) -> Move {
        match (self, other) {
            (GameResult::Win, Move::Rock) => Move::Paper,
            (GameResult::Win, Move::Paper) => Move::Scissors,
            (GameResult::Win, Move::Scissors) => Move::Rock,
            (GameResult::Lose, Move::Rock) => Move::Scissors,
            (GameResult::Lose, Move::Paper) => Move::Rock,
            (GameResult::Lose, Move::Scissors) => Move::Paper,
            (GameResult::Draw, m) => *m,
        }
    }

    fn score(&self, other: &Move) -> Score {
        *self as Score + self.move_for(other) as Score
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(s.to_string()),
        }
    }
}

impl FromStr for GameResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err(s.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1() {
        assert_eq!(Day2::part1(TEST_INPUT), 15);
    }

    #[test]
    fn part2() {
        assert_eq!(Day2::part2(TEST_INPUT), 12)
    }
}
