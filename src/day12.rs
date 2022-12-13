use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    error::Error,
    fmt::{self, Debug},
    ops::Deref,
    str::FromStr,
};

#[allow(dead_code)]
static INPUT: &str = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;

pub fn run() {
    let input = include_str!("../input/day12/input");
    // let hm: HeightMap = input.parse().unwrap();
    dbg!(first(input));
    dbg!(second(input));

    let hm: HeightMap = INPUT.parse().unwrap();
    // dbg!(&hm);
    println!("{}", &hm);

    let path = hm.shortest_path();
    dbg!(&path.inner.len());
    hm.print_path(&path);
    // dbg!(&path.inner.len());
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Ord for Position {
    // Reading order: Y then X
    fn cmp(&self, other: &Self) -> Ordering {
        (self.y, self.x).cmp(&(other.y, other.x))
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn distance_squared(&self, other: Self) -> usize {
        let dx = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };
        let dy = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };
        dx * dx + dy * dy
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
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct HeightMap {
    start: Position,
    end: Option<Position>,
    grid: Vec<Vec<u8>>,
}

#[derive(Debug, Default, Clone)]
pub struct ClimbPath {
    inner: Vec<Position>,
    directions: Vec<Direction>,
}

// Generic struct used to select an item based on a minimum score.
// Use with std::collections::BinaryHeap for problems requiring Djikstra's
// Algorithm or A*

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
pub struct ScoredItem<N, T> {
    pub cost: N,
    pub item: T,
}

impl<N: Ord + PartialOrd, T: Ord + PartialOrd> Ord for ScoredItem<N, T> {
    fn cmp(&self, other: &ScoredItem<N, T>) -> Ordering {
        (&other.cost, &other.item).cmp(&(&self.cost, &self.item))
    }
}

impl<N: Ord + PartialOrd, T: Ord + PartialOrd> PartialOrd for ScoredItem<N, T> {
    fn partial_cmp(&self, other: &ScoredItem<N, T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl HeightMap {
    #[inline]
    fn row_len(&self) -> usize {
        self.grid[0].len()
    }

    #[inline]
    fn col_len(&self) -> usize {
        self.grid.len()
    }

    #[inline]
    fn pos_item(&self, pos: impl Into<Position>) -> u8 {
        let p = pos.into();
        self.grid[p.y][p.x]
    }

    #[inline]
    fn can_climb(&self, cur: &Position, next: &Position) -> bool {
        (self.pos_item(*next) == self.pos_item(*cur))
            || (self.pos_item(*next) == self.pos_item(*cur) + 1)
    }

    fn neightbors(&self, pos: &Position) -> Vec<(Position, Direction)> {
        let mut result = vec![];
        if pos.x > 0 {
            let left = Position::new(pos.x - 1, pos.y);
            result.push((left, Direction::Left));
        }
        if pos.x < self.row_len() - 1 {
            let right = Position::new(pos.x + 1, pos.y);
            result.push((right, Direction::Right));
        }
        // upside down
        if pos.y > 0 {
            let up = Position::new(pos.x, pos.y - 1);
            result.push((up, Direction::Up));
        }
        if pos.y < self.col_len() - 1 {
            let down = Position::new(pos.x, pos.y + 1);
            result.push((down, Direction::Down));
        }
        result
    }

    fn shortest_path(&self) -> ClimbPath {
        let mut queue = BinaryHeap::from([ScoredItem {
            cost: 0,
            item: self.start,
        }]);
        let mut visited = HashSet::new();
        let mut came_from: HashMap<Position, (Position, Direction)> = HashMap::new();
        let mut climb_path = ClimbPath::default();

        while !queue.is_empty() {
            let step = queue.pop().unwrap();
            if visited.contains(&step.item) {
                continue;
            }
            if self.end == Some(step.item)
                || (self.end.is_none() && self.pos_item(step.item) == my_char_to_u8('z'))
            {
                let mut curr = step.item;
                while curr != self.start {
                    let f = came_from.get(&curr).unwrap().to_owned();
                    climb_path.inner.push(curr);
                    climb_path.directions.push(f.1);
                    curr = f.0;
                }
                climb_path.inner.reverse();
                climb_path.directions.reverse();
                return climb_path;
            }

            visited.insert(step.item);
            for n in self.neightbors(&step.item) {
                if self.can_climb(&step.item, &n.0) {
                    queue.push(ScoredItem {
                        cost: step.cost + 1,
                        item: n.0,
                    });
                    if !visited.contains(&n.0) {
                        came_from.insert(n.0, (step.item, n.1));
                    }
                }
            }
        }
        climb_path
    }

    fn print_path(&self, path: &ClimbPath) {
        for col in 0..self.col_len() {
            for row in 0..self.row_len() {
                let pos = Position::new(row, col);
                if let Some(idx) = [self.start]
                    .iter()
                    .chain(path.inner.iter())
                    .take(path.inner.len())
                    .position(|i| i == &pos)
                {
                    match path.directions[idx] {
                        Direction::Up => print!("^"),
                        Direction::Down => print!("v"),
                        Direction::Left => print!("<"),
                        Direction::Right => print!(">"),
                    }
                } else if Some(pos) == self.end {
                    print!("E");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

impl Deref for HeightMap {
    type Target = Vec<Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl fmt::Display for HeightMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}, {:?}", self.start, self.end)?;
        for col in 0..self.col_len() {
            for row in 0..self.row_len() {
                let pos = Position::new(row, col);
                if pos == self.start {
                    write!(f, "S")?;
                } else if Some(pos) == self.end {
                    write!(f, "E")?;
                } else {
                    write!(f, "{}", my_u8_to_char(self.pos_item(pos)))?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[inline]
fn my_char_to_u8(c: char) -> u8 {
    assert!(c.is_ascii_lowercase());
    c as u8 - b'a'
}

#[inline]
fn my_u8_to_char(n: u8) -> char {
    assert!(n < 26);
    (n + b'a') as char
}

/// An error returned when parsing a `bool` using [`from_str`] fails
///
/// [`from_str`]: super::FromStr::from_str
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ParseGridError;

impl fmt::Display for ParseGridError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "provided string was not in allow list")
    }
}

impl Error for ParseGridError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "failed to parse cmd"
    }
}

impl FromStr for HeightMap {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = Position::default();
        let mut end = Position::default();
        let grid = s
            .split('\n')
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| match c {
                        'S' => {
                            start = Position { x: col, y: row };
                            my_char_to_u8('a')
                        }
                        'E' => {
                            end = Position { x: col, y: row };
                            my_char_to_u8('z')
                        }
                        _ => my_char_to_u8(c),
                    })
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();
        Ok(Self {
            start,
            end: Some(end),
            grid,
        })
    }
}

fn first(input: &str) -> usize {
    let hm: HeightMap = input.parse().unwrap();
    let climb_path = hm.shortest_path();
    climb_path.inner.len()
}

fn second(input: &str) -> usize {
    let origin: HeightMap = input.parse().unwrap();
    let grid = origin
        .grid
        .iter()
        .map(|i| i.iter().map(|j| my_char_to_u8('z') - j).collect())
        .collect();
    let hm = HeightMap {
        start: origin.end.unwrap(),
        end: None,
        grid,
    };
    let climb_path = hm.shortest_path();
    climb_path.inner.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(first(INPUT), 31);
    }

    #[test]
    fn test_second() {
        assert_eq!(second(INPUT), 29);
    }
}
