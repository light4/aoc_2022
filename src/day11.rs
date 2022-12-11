use std::{
    collections::VecDeque,
    error::Error,
    fmt::{self, Debug},
    str::FromStr,
};

#[allow(dead_code)]
static INPUT: &str = r#"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"#;

pub fn run() {
    let input = include_str!("../input/day11/input");
    dbg!(first(input));
    dbg!(second(input));
}

#[derive(Debug, Default, Clone)]
pub struct Machine {
    monkeys: Vec<Monkey>,
    round: usize,
    inspects: Vec<usize>,
    part: Part,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Part {
    #[default]
    One,
    Two,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test_div: usize,
    true_to: usize,
    false_to: usize,
}

#[derive(Debug, Clone)]
pub struct Operation {
    op: OP,
    operand: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum OP {
    Add,
    AddAssign,
    Mul,
    MulAssign,
}

impl Operation {
    #[inline]
    fn to_func(&self) -> Box<dyn Fn(usize) -> usize + '_> {
        match self.op {
            OP::Add => Box::new(move |x| x + self.operand.unwrap()),
            OP::AddAssign => Box::new(move |x| x + x),
            OP::Mul => Box::new(move |x| x * self.operand.unwrap()),
            OP::MulAssign => Box::new(move |x| x * x),
        }
    }
}

impl FromStr for Operation {
    type Err = ParseMonkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let operation_str_vec: Vec<&str> = s
            .split('=')
            .nth(1)
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .collect();
        let operation = match (operation_str_vec[1], operation_str_vec[2]) {
            ("+", "old") => Operation {
                op: OP::AddAssign,
                operand: None,
            },
            ("*", "old") => Operation {
                op: OP::MulAssign,
                operand: None,
            },
            ("+", n) => Operation {
                op: OP::Add,
                operand: Some(n.parse().unwrap()),
            },
            ("*", n) => Operation {
                op: OP::Mul,
                operand: Some(n.parse().unwrap()),
            },
            _ => return Err(ParseMonkeyError),
        };
        Ok(operation)
    }
}
/// An error returned when parsing a `bool` using [`from_str`] fails
///
/// [`from_str`]: super::FromStr::from_str
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ParseMonkeyError;

impl fmt::Display for ParseMonkeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "provided string was not a monkey")
    }
}

impl Error for ParseMonkeyError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "failed to parse monkey"
    }
}

fn last_num(s: &str) -> usize {
    s.split_ascii_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap()
}

impl FromStr for Monkey {
    type Err = ParseMonkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s
            .lines()
            .skip(1)
            .map(|i| i.trim())
            .filter(|i| !i.is_empty())
            .collect();
        let item_num_str = lines[0].split(':').nth(1).unwrap();
        let items = item_num_str
            .split(',')
            .map(|n| n.trim().parse::<usize>().unwrap())
            .collect::<VecDeque<usize>>();

        let operation = lines[1].parse()?;
        let test_div = last_num(lines[2]);
        let true_to = last_num(lines[3]);
        let false_to = last_num(lines[4]);
        Ok(Self {
            items,
            operation,
            test_div,
            true_to,
            false_to,
        })
    }
}

impl Machine {
    #[inline]
    fn with_monkeys(mut self, monkeys: Vec<Monkey>) -> Self {
        let count = monkeys.len();
        self.monkeys = monkeys;
        self.inspects = vec![0; count];
        self
    }

    #[inline]
    fn with_part(mut self, part: Part) -> Self {
        self.part = part;
        self
    }

    fn part_one_lower_worry_levels(&self, l: usize) -> usize {
        l / 3
    }

    fn part_two_lower_worry_levels(&self, l: usize) -> usize {
        let div_by: usize = self.monkeys.iter().map(|i| i.test_div).product();
        l % div_by
    }

    fn run_once(&mut self) {
        for i in 0..self.monkeys.len() {
            while let Some(worry_level) = self.monkeys[i].items.pop_front() {
                self.inspects[i] += 1;
                let true_to = self.monkeys[i].true_to;
                let false_to = self.monkeys[i].false_to;
                let mut new_worry_level = self.monkeys[i].operation.to_func()(worry_level);
                new_worry_level = match &self.part {
                    Part::One => self.part_one_lower_worry_levels(new_worry_level),
                    Part::Two => self.part_two_lower_worry_levels(new_worry_level),
                };
                if new_worry_level % self.monkeys[i].test_div == 0 {
                    self.monkeys[true_to].items.push_back(new_worry_level);
                } else {
                    self.monkeys[false_to].items.push_back(new_worry_level);
                }
            }
        }
    }

    fn run_until_round(&mut self, round: usize) {
        for _ in (self.round + 1)..=round {
            self.run_once();
        }
    }

    fn monkey_business(&mut self, round: usize) -> usize {
        self.run_until_round(round);
        let mut inspects = self.inspects.clone();
        inspects.sort();
        inspects.reverse();
        inspects[0] * inspects[1]
    }
}

fn to_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|i| i.trim())
        .filter(|i| !i.is_empty())
        .map(|i| i.parse().unwrap())
        .collect()
}

fn first(input: &str) -> usize {
    let monkeys = to_monkeys(input);
    let mut m = Machine::default().with_monkeys(monkeys);
    m.monkey_business(20)
}

fn second(input: &str) -> usize {
    let monkeys = to_monkeys(input);
    let mut m = Machine::default()
        .with_monkeys(monkeys)
        .with_part(Part::Two);
    m.monkey_business(10000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(first(INPUT), 10605);
    }

    #[test]
    fn test_second() {
        assert_eq!(second(INPUT), 2713310158);
    }
}
