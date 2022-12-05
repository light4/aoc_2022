use std::{
    num::ParseIntError,
    ops::{Deref, DerefMut},
    str::FromStr,
};

pub fn run() {
    let input = include_str!("../input/day05/input");
    dbg!(first(input));
    dbg!(second(input));
}

#[derive(Default, Debug, Clone)]
pub struct Stack(Vec<char>);

impl Deref for Stack {
    type Target = Vec<char>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Debug, Clone)]
pub struct Crate(Vec<Stack>);

impl Deref for Crate {
    type Target = Vec<Stack>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Crate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Crate {
    fn move_action(&mut self, action: &str) {
        // "move 1 from 2 to 1"
        let splited: Vec<&str> = action.split_whitespace().collect();
        let (num, from, to) = (
            splited[1].parse::<usize>().unwrap(),
            splited[3].parse::<usize>().unwrap(),
            splited[5].parse::<usize>().unwrap(),
        );
        for _ in 0..num {
            let need_move = self.0[from - 1].pop().unwrap();
            self.0[to - 1].push(need_move);
        }
    }

    fn new_move_action(&mut self, action: &str) {
        // "move 1 from 2 to 1"
        let splited: Vec<&str> = action.split_whitespace().collect();
        let (num, from, to) = (
            splited[1].parse::<usize>().unwrap(),
            splited[3].parse::<usize>().unwrap(),
            splited[5].parse::<usize>().unwrap(),
        );
        let need_moved = self.0[from - 1][(self.0[from - 1].len() - num)..].to_owned();
        for c in need_moved {
            self.0[from - 1].pop();
            self.0[to - 1].push(c);
        }
    }
}

impl FromStr for Crate {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split('\n').filter(|l| !l.is_empty()).collect();
        let stack_count: usize = lines
            .last()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        dbg!(stack_count);
        let mut _crate = Crate::default();
        for i in 0..stack_count {
            let mut stack = Stack::default();
            for line in lines.iter().rev() {
                if let Some(c) = line.char_indices().nth(i * 4 + 1) {
                    if c.1.is_ascii_uppercase() {
                        stack.push(c.1);
                    }
                }
            }
            _crate.push(stack);
        }
        Ok(_crate)
    }
}

fn first(input: &str) -> String {
    let splited: Vec<&str> = input.split("\n\n").collect();
    dbg!(&splited[0]);
    let mut _crate: Crate = splited[0].parse().unwrap();
    dbg!(&_crate);

    let moves: Vec<&str> = splited[1]
        .split('\n')
        .map(|k| k.trim())
        .filter(|j| !j.is_empty())
        .collect();

    for action in moves {
        _crate.move_action(action);
    }

    _crate
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.last().unwrap())
        .copied()
        .collect()
}

fn second(input: &str) -> String {
    let splited: Vec<&str> = input.split("\n\n").collect();
    dbg!(&splited[0]);
    let mut _crate: Crate = splited[0].parse().unwrap();
    dbg!(&_crate);

    let moves: Vec<&str> = splited[1]
        .split('\n')
        .map(|k| k.trim())
        .filter(|j| !j.is_empty())
        .collect();

    for action in moves {
        _crate.new_move_action(action);
    }

    _crate
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.last().unwrap())
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
    ";

    #[test]
    fn test_first() {
        assert_eq!(first(INPUT), "CMZ");
    }

    #[test]
    fn test_second() {
        assert_eq!(second(INPUT), "MCD");
    }
}
