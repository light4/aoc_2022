use std::{collections::HashSet, num::ParseIntError, str::FromStr};

pub fn run() {
    let input = include_str!("../input/day04/first");
    // let input: &str = "
    //     2-4,6-8
    //     2-3,4-5
    //     5-7,7-9
    //     2-8,3-7
    //     6-6,4-6
    //     2-6,4-8
    // ";

    // dbg!(first(input));
    dbg!(second(input));
}

#[derive(Debug, Clone, Copy)]
pub struct Range {
    start: usize,
    end: usize,
}

impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num: Vec<&str> = s.split('-').collect();
        let start = num[0].parse()?;
        let end = num[1].parse()?;
        Ok(Self { start, end })
    }
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        (self.start <= other.start) && (self.end >= other.end)
    }

    fn overlap(&self, other: &Self) -> bool {
        (self.start >= other.start && self.start <= other.end)
            || (self.end >= other.start && self.end <= other.end)
            || self.contains(other)
            || other.contains(self)
    }
}

fn first(input: &str) -> usize {
    input
        .split("\n")
        .map(|k| k.trim())
        .filter(|j| !j.is_empty())
        .map(|i| {
            let pairs: Vec<&str> = i.split(',').take(2).collect();
            let left: Range = pairs[0].parse().unwrap();
            let right: Range = pairs[1].parse().unwrap();
            return (left, right);
        })
        .filter(|(left, right)| left.contains(right) || right.contains(left))
        .count()
}

fn second(input: &str) -> usize {
    input
        .split("\n")
        .map(|k| k.trim())
        .filter(|j| !j.is_empty())
        .map(|i| {
            let pairs: Vec<&str> = i.split(',').take(2).collect();
            let left: Range = pairs[0].parse().unwrap();
            let right: Range = pairs[1].parse().unwrap();
            return (left, right);
        })
        .filter(|(left, right)| left.overlap(right))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    ";

    #[test]
    fn test_first() {
        assert_eq!(first(INPUT), 2);
    }

    #[test]
    fn test_second() {
        assert_eq!(second(INPUT), 4);
    }
}
