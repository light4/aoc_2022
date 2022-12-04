use std::collections::HashSet;

pub fn run() {
    let input = include_str!("../input/day03/first");
    dbg!(first(input));
    dbg!(second(input));
}

fn first(input: &str) -> usize {
    input
        .split('\n')
        .map(|k| k.trim())
        .filter(|j| !j.is_empty())
        .map(|j| {
            let split_idx = j.len() / 2;
            let left: HashSet<char> = j[0..split_idx].chars().collect();
            let right: HashSet<char> = j[split_idx..].chars().collect();
            let c = left.intersection(&right).into_iter().next().unwrap();
            c.to_owned()
        })
        .map(|c| match c {
            'a'..='z' => (c as u8 - 97 + 1) as usize,
            'A'..='Z' => (c as u8 - 65 + 27) as usize,
            _ => unreachable!(),
        })
        .sum::<usize>()
}

fn second(input: &str) -> usize {
    let groups: Vec<&str> = input
        .split('\n')
        .map(|k| k.trim())
        .filter(|j| !j.is_empty())
        .collect();

    groups
        .chunks(3)
        .map(|j| {
            let first: HashSet<char> = j[0].chars().collect();
            let second: HashSet<char> = j[1].chars().collect();
            let third: HashSet<char> = j[2].chars().collect();
            let fs: HashSet<char> = first.intersection(&second).copied().collect();
            let c = fs.intersection(&third).copied().next().unwrap();
            c
        })
        .map(|c| match c {
            'a'..='z' => (c as u8 - 97 + 1) as usize,
            'A'..='Z' => (c as u8 - 65 + 27) as usize,
            _ => unreachable!(),
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    ";

    #[test]
    fn test_first() {
        assert_eq!(first(INPUT), 157);
    }

    #[test]
    fn test_second() {
        assert_eq!(second(INPUT), 70);
    }
}
