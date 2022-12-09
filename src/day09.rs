use std::{collections::HashSet, error::Error, fmt, str::FromStr};

pub fn run() {
    // let input = r#"
    //     R 5
    //     U 8
    //     L 8
    //     D 3
    //     R 17
    //     D 10
    //     L 25
    //     U 20
    // "#;
    let input = include_str!("../input/day09/input");
    dbg!(first(input));
    dbg!(second(input));
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: isize,
    y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn is_touching(&self, other: &Self) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    pub fn move_towards(&self, other: &Self) -> Option<Self> {
        if self.is_touching(other) {
            return None;
        }

        Some(Position {
            x: self.x + (other.x - self.x).signum(),
            y: self.y + (other.y - self.y).signum(),
        })
    }
}

impl From<(isize, isize)> for Position {
    fn from(value: (isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<&(isize, isize)> for Position {
    fn from(value: &(isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum Motion {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

/// An error returned when parsing a `bool` using [`from_str`] fails
///
/// [`from_str`]: super::FromStr::from_str
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ParseMotionError;

impl fmt::Display for ParseMotionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "provided string was not a motion".fmt(f)
    }
}

impl Error for ParseMotionError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "failed to parse motion"
    }
}

impl FromStr for Motion {
    type Err = ParseMotionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splited: Vec<&str> = s.split_ascii_whitespace().collect();
        match splited[0] {
            "U" => Ok(Self::Up(splited[1].parse().unwrap())),
            "D" => Ok(Self::Down(splited[1].parse().unwrap())),
            "L" => Ok(Self::Left(splited[1].parse().unwrap())),
            "R" => Ok(Self::Right(splited[1].parse().unwrap())),
            _ => Err(ParseMotionError),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Rope {
    knots: Vec<Position>,
    tail_path: Vec<Position>,
    left: isize,
    right: isize,
    up: isize,
    down: isize,
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let knots_len = self.knots.len();
        for j in (self.down..=self.up.max(5)).rev() {
            for i in self.left..=self.right.max(5) {
                let pos = Position::new(i, j);
                if self.knots.contains(&pos) {
                    let idx = self.knots.iter().position(|i| i == &pos).unwrap();
                    if idx == 0 {
                        "H".fmt(f)?;
                    } else if knots_len == 2 && idx == 1 {
                        // part 1
                        "T".fmt(f)?;
                    } else {
                        write!(f, "{idx}")?;
                    }
                } else if i == 0 && j == 0 {
                    write!(f, "s")?;
                } else {
                    write!(f, ".")?;
                }
            }
            "\n".fmt(f)?;
        }
        "\n".fmt(f)?;

        Ok(())
    }
}

impl Rope {
    fn with_knots(n: usize) -> Self {
        Self {
            knots: vec![Position::default(); n + 1],
            tail_path: vec![Position::default()],
            ..Default::default()
        }
    }

    fn move_motion(&mut self, motion: &Motion) {
        match motion {
            Motion::Up(n) => {
                for _ in 0..*n {
                    self.move_once(&Direction::Up);
                }
            }
            Motion::Down(n) => {
                for _ in 0..*n {
                    self.move_once(&Direction::Down);
                }
            }
            Motion::Left(n) => {
                for _ in 0..*n {
                    self.move_once(&Direction::Left);
                }
            }
            Motion::Right(n) => {
                for _ in 0..*n {
                    self.move_once(&Direction::Right);
                }
            }
        }
    }

    fn move_once(&mut self, direct: &Direction) {
        match direct {
            Direction::Up => {
                self.knots[0] = Position::new(self.knots[0].x, self.knots[0].y + 1);
                self.up = self.up.max(self.knots[0].y);
            }
            Direction::Down => {
                self.knots[0] = Position::new(self.knots[0].x, self.knots[0].y - 1);
                self.down = self.down.min(self.knots[0].y);
            }
            Direction::Left => {
                self.knots[0] = Position::new(self.knots[0].x - 1, self.knots[0].y);
                self.left = self.left.min(self.knots[0].x);
            }
            Direction::Right => {
                self.knots[0] = Position::new(self.knots[0].x + 1, self.knots[0].y);
                self.right = self.right.max(self.knots[0].x);
            }
        };
        let before = self.knots.clone();
        let mut prev = before[0];
        for (idx, knot) in before.iter().enumerate().skip(1) {
            if let Some(p) = knot.move_towards(&prev) {
                self.knots[idx] = p;
                if idx == before.len() - 1 {
                    self.tail_path.push(p);
                }
                prev = p;
            } else {
                break;
            }
        }
        // println!("{self}");
    }
}

fn to_motions(input: &str) -> Vec<Motion> {
    input
        .split('\n')
        .map(|i| i.trim())
        .filter(|i| !i.is_empty())
        .map(|i| i.parse().unwrap())
        .collect()
}

fn first(input: &str) -> usize {
    let motions: Vec<Motion> = to_motions(input);
    let mut rope = Rope::with_knots(1);
    for motion in motions {
        rope.move_motion(&motion);
    }
    rope.tail_path
        .into_iter()
        .collect::<HashSet<Position>>()
        .len()
}

fn second(input: &str) -> usize {
    let motions: Vec<Motion> = to_motions(input);
    let mut rope = Rope::with_knots(9);
    for motion in motions {
        rope.move_motion(&motion);
    }
    rope.tail_path
        .into_iter()
        .collect::<HashSet<Position>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_touching() {
        let head = Position::new(3, 3);
        let tail = Position::new(2, 4);
        assert!(tail.is_touching(&head));
    }

    #[test]
    fn test_move_towards() {
        let head = Position::new(3, 3);
        let tail = Position::new(2, 4);
        let after = tail.move_towards(&head);
        assert!(after.is_none());

        let head = Position::new(4, 3);
        let tail = Position::new(2, 4);
        let after = tail.move_towards(&head);
        assert_eq!(after, Some(Position::new(3, 3)));

        let head = Position::new(3, 2);
        let tail = Position::new(4, 3);
        let after = tail.move_towards(&head);
        assert!(after.is_none());
    }

    #[test]
    fn test_first() {
        let input = "
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2
        ";
        assert_eq!(first(input), 13);
    }

    #[test]
    fn test_second() {
        let input = r#"
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
        "#;
        assert_eq!(second(input), 36);
    }
}
