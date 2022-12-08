use std::{error::Error, fmt, ops::Deref, str::FromStr};

pub fn run() {
    let input = include_str!("../input/day08/input");
    dbg!(first(input));
    dbg!(second(input));
}

pub const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::Top,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<&(usize, usize)> for Position {
    fn from(value: &(usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Top,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Grid(Vec<Vec<u8>>);

impl Grid {
    #[inline]
    fn row_len(&self) -> usize {
        self.0[0].len()
    }

    #[inline]
    fn col_len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn pos_item(&self, pos: impl Into<Position>) -> u8 {
        let p = pos.into();
        self.0[p.y][p.x]
    }

    fn neighbors(&self, pos: &Position, direct: Direction) -> Vec<u8> {
        match direct {
            Direction::Top => (0..pos.y)
                .into_iter()
                .map(|i| self.pos_item((pos.x, i)))
                .rev()
                .collect(),
            Direction::Down => ((pos.y + 1)..self.col_len())
                .into_iter()
                .map(|i| self.pos_item((pos.x, i)))
                .collect(),
            Direction::Left => (0..pos.x)
                .into_iter()
                .map(|i| self.pos_item((i, pos.y)))
                .rev()
                .collect(),
            Direction::Right => ((pos.x + 1)..self.row_len())
                .into_iter()
                .map(|i| self.pos_item((i, pos.y)))
                .collect(),
        }
    }

    fn is_visible(&self, pos: &Position) -> bool {
        if pos.x == 0
            || (pos.x == self.row_len() - 1)
            || pos.y == 0
            || (pos.y == self.col_len() - 1)
        {
            return true;
        }
        let item = self.pos_item(*pos);
        ALL_DIRECTIONS
            .into_iter()
            .any(|direct| self.neighbors(pos, direct).into_iter().all(|i| i < item))
    }

    fn scenic_score(&self, pos: &Position) -> usize {
        let item = self.pos_item(*pos);
        ALL_DIRECTIONS
            .into_iter()
            .map(|direct| {
                let ns = self.neighbors(pos, direct);
                let v = &ns.iter().copied().take_while(|i| *i < item).count() + 1;
                v.clamp(1, ns.len())
            })
            .product()
    }

    fn all_visible_count(&self) -> usize {
        (0..self.col_len())
            .into_iter()
            .flat_map(move |x| {
                (0..self.row_len())
                    .into_iter()
                    .filter(move |y| self.is_visible(&(x, *y).into()))
            })
            .count()
    }

    fn highest_scenic_score(&self) -> usize {
        (1..(self.col_len() - 1))
            .into_iter()
            .flat_map(move |x| {
                (1..(self.row_len() - 1))
                    .into_iter()
                    .map(move |y| self.scenic_score(&(x, y).into()))
            })
            .max()
            .unwrap()
    }
}

impl Deref for Grid {
    type Target = Vec<Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for col in row {
                write!(f, "{col}")?;
            }
            "\n".fmt(f)?;
        }
        Ok(())
    }
}

/// An error returned when parsing a `bool` using [`from_str`] fails
///
/// [`from_str`]: super::FromStr::from_str
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ParseGridError;

impl fmt::Display for ParseGridError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "provided string was not in allow list".fmt(f)
    }
}

impl Error for ParseGridError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "failed to parse cmd"
    }
}

impl FromStr for Grid {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = s
            .split('\n')
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();
        Ok(Self(result))
    }
}

fn first(input: &str) -> usize {
    let grid: Grid = input.parse().unwrap();
    grid.all_visible_count()
}

fn second(input: &str) -> usize {
    let grid: Grid = input.parse().unwrap();
    grid.highest_scenic_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "
        30373
        25512
        65332
        33549
        35390
    ";

    #[test]
    fn test_first() {
        assert_eq!(first(INPUT), 21);
    }

    #[test]
    fn test_second() {
        assert_eq!(second(INPUT), 8);
    }
}
