use std::{cmp::Ordering, error::Error, fmt, str::FromStr};

pub fn run() {
    let input = include_str!("../input/day02/first");
    // dbg!(first(input));
    dbg!(second(input));
}

#[derive(Debug, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        match (&self, other) {
            (Self::Rock, Self::Rock) => Ordering::Equal,
            (Self::Paper, Self::Paper) => Ordering::Equal,
            (Self::Scissors, Self::Scissors) => Ordering::Equal,

            (Self::Rock, Self::Scissors) => Ordering::Greater,
            (Self::Paper, Self::Rock) => Ordering::Greater,
            (Self::Scissors, Self::Paper) => Ordering::Greater,

            (Self::Rock, Self::Paper) => Ordering::Less,
            (Self::Paper, Self::Scissors) => Ordering::Less,
            (Self::Scissors, Self::Rock) => Ordering::Less,
        }
    }

    fn compare(&self, other: &Self) -> usize {
        match self.cmp(other) {
            Ordering::Less => self.score() + 0,
            Ordering::Equal => self.score() + 3,
            Ordering::Greater => self.score() + 6,
        }
    }

    fn less(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn greater(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
}

/// An error returned when parsing a `bool` using [`from_str`] fails
///
/// [`from_str`]: super::FromStr::from_str
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ParseChoiceError;

impl fmt::Display for ParseChoiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "provided string was not in allow list".fmt(f)
    }
}

impl Error for ParseChoiceError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "failed to parse bool"
    }
}

impl FromStr for Choice {
    type Err = ParseChoiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),

            "X" => Ok(Self::Rock),
            "Y" => Ok(Self::Paper),
            "Z" => Ok(Self::Scissors),

            _ => Err(ParseChoiceError),
        }
    }
}

fn first(input: &str) -> usize {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|i| {
            let pair = i
                .trim()
                .split_ascii_whitespace()
                .take(2)
                .collect::<Vec<&str>>();
            let left: Choice = pair[0].parse().unwrap();
            let right: Choice = pair[1].parse().unwrap();
            return (left, right);
        })
        .map(|(left, right)| right.compare(&left))
        .sum()
}

fn second(input: &str) -> usize {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|i| {
            let pair = i
                .trim()
                .split_ascii_whitespace()
                .take(2)
                .collect::<Vec<&str>>();
            let left: Choice = pair[0].parse().unwrap();
            let right = match pair[1] {
                "X" => left.less(),
                "Y" => left,
                "Z" => left.greater(),
                _ => unreachable!(),
            };
            return (left, right);
        })
        .map(|(left, right)| right.compare(&left))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "
    A Y
    B X
    C Z";

    #[test]
    fn test_first() {
        assert_eq!(first(INPUT), 15);
    }

    #[test]
    fn test_second() {
        assert_eq!(second(INPUT), 12);
    }
}
