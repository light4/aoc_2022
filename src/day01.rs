pub fn run() {
    let input = include_str!("../input/day01/first");
    // dbg!(first(input));
    dbg!(second(input));
}

fn first(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|i| {
            i.split("\n")
                .filter(|j| !j.is_empty())
                .map(|k| k.trim())
                .map(|j| j.parse::<usize>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}

fn second(input: &str) -> usize {
    let mut calories: Vec<usize> = input
        .split("\n\n")
        .map(|i| {
            i.split("\n")
                .filter(|j| !j.is_empty())
                .map(|k| k.trim())
                .map(|j| j.parse::<usize>().unwrap())
                .sum()
        })
        .collect();
    calories.sort_by(|a, b| b.cmp(a));
    dbg!(&calories);
    calories.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "
    1000
    2000
    3000

    4000

    5000
    6000

    7000
    8000
    9000

    10000";
    #[test]
    fn test_first() {
        assert_eq!(first(INPUT), 24000);
    }

    #[test]
    fn test_second() {
        assert_eq!(second(INPUT), 45000);
    }
}
