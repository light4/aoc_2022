use std::collections::HashSet;

pub fn run() {
    let input = include_str!("../input/day06/input").trim();
    dbg!(first(input));
    dbg!(second(input));
}

fn first(input: &str) -> usize {
    let marker = 4;
    let test: Vec<char> = input.chars().collect();
    for (idx, item) in test.windows(marker).enumerate() {
        let set: HashSet<char> = HashSet::from_iter(item.iter().copied());
        if set.len() == marker {
            return idx + marker;
        }
    }
    0
}

fn second(input: &str) -> usize {
    let marker = 14;
    let test: Vec<char> = input.chars().collect();
    for (idx, item) in test.windows(marker).enumerate() {
        let set: HashSet<char> = HashSet::from_iter(item.iter().copied());
        if set.len() == marker {
            return idx + marker;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(first("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(first("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(first("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(first("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_second() {
        assert_eq!(second("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(second("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(second("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(second("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(second("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
