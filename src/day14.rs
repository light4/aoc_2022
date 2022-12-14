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
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"#;

pub fn run() {
    let input = include_str!("../input/day14/input");
    // dbg!(first(input));
    // dbg!(second(input));

    let mut mm = MineMap::default()
        .with_grid(vec![vec![State::default(); 600]; 600])
        .with_start((500, 0));
    for s in input
        .lines()
        .filter_map(|i| if !i.is_empty() { Some(i.trim()) } else { None })
    {
        mm.update_rock_path(s);
    }

    let mut result = vec![];
    while let Some(p) = mm.run_once() {
        result.push(p);
    }
    println!("{mm}");
    dbg!(result.len());
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum State {
    #[default]
    Air,
    Rock,
    Sand,
    RestSand,
    Start,
}

#[derive(Debug, Clone, Default)]
pub struct MineMap {
    start: Position,
    grid: Vec<Vec<State>>,
    left_edge: usize,
    right_edge: usize,
    height: usize,
}

fn line_to_points(one: Position, two: Position) -> Vec<Position> {
    let start_x = one.x.min(two.x);
    let end_x = one.x.max(two.x);
    let start_y = one.y.min(two.y);
    let end_y = one.y.max(two.y);
    let mut result = vec![];
    for i in start_x..=end_x {
        for j in start_y..=end_y {
            result.push(Position::new(i, j));
        }
    }
    result
}

fn to_rock_points(s: &str) -> Vec<Position> {
    let path_points: Vec<Position> = s
        .split("->")
        .map(|i| {
            let splited: Vec<usize> = i
                .split(',')
                .map(|i| i.trim())
                .map(|i| i.parse().unwrap())
                .collect();
            Position::new(splited[0], splited[1])
        })
        .collect();
    let rock_points: Vec<Position> = path_points
        .windows(2)
        .map(|pairs| line_to_points(pairs[0], pairs[1]))
        .flatten()
        .collect();
    rock_points
}

impl MineMap {
    #[inline]
    fn row_len(&self) -> usize {
        self.grid[0].len()
    }

    #[inline]
    fn col_len(&self) -> usize {
        self.grid.len()
    }

    #[inline]
    fn pos_item(&self, pos: impl Into<Position>) -> State {
        let p = pos.into();
        self.grid[p.y][p.x]
    }

    #[inline]
    fn update_point(&mut self, pos: impl Into<Position>, state: State) {
        let pos = pos.into();
        self.left_edge = self.left_edge.min(pos.x);
        self.right_edge = self.right_edge.max(pos.x);
        self.height = self.height.max(pos.y);
        self.grid[pos.y][pos.x] = state;
    }

    #[inline]
    fn with_start(mut self, start: impl Into<Position>) -> Self {
        self.start = start.into();
        self.left_edge = self.start.x;
        self.update_point(self.start, State::Start);
        self
    }

    #[inline]
    fn with_grid(mut self, grid: Vec<Vec<State>>) -> Self {
        self.grid = grid;
        self
    }

    #[inline]
    fn pos_item_mut(&mut self, pos: impl Into<Position>) -> &mut State {
        let p = pos.into();
        &mut self.grid[p.y][p.x]
    }

    fn update_rock_path(&mut self, s: &str) {
        let rock_points = to_rock_points(s);
        for p in rock_points {
            self.update_point(p, State::Rock);
        }
    }

    fn fall_path(&self, cur: &Position) -> Option<Position> {
        let possible_pos: [Position; 3] = [
            (cur.x, cur.y + 1).into(),
            (cur.x - 1, cur.y + 1).into(),
            (cur.x + 1, cur.y + 1).into(),
        ];
        for p in possible_pos {
            if self.pos_item(p) == State::Air {
                return Some(p);
            }
        }
        None
    }

    fn run_once(&mut self) -> Option<Position> {
        let mut cur = self.start;
        while let Some(p) = self.fall_path(&cur) {
            if p.y > self.height {
                return None;
            }
            cur = p
        }
        self.update_point(cur, State::RestSand);
        Some(cur)
    }
}

impl Deref for MineMap {
    type Target = Vec<Vec<State>>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl fmt::Display for MineMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let context = 3;
        writeln!(f, "{:?}", self.start)?;
        for col in 0..=self.height.wrapping_add(context) {
            write!(f, "{:3} ", col)?;
            for row in self.left_edge.wrapping_sub(context)..=self.right_edge.wrapping_add(context)
            {
                let pos = Position::new(row, col);
                let item = self.pos_item(pos);
                match item {
                    State::Air => write!(f, ".")?,
                    State::Rock => write!(f, "#")?,
                    State::RestSand => write!(f, "o")?,
                    State::Sand => write!(f, "~")?,
                    State::Start => write!(f, "+")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn first(input: &str) -> usize {
    let mut mm = MineMap::default()
        .with_grid(vec![vec![State::default(); 600]; 600])
        .with_start((500, 0));
    for s in input
        .lines()
        .filter_map(|i| if !i.is_empty() { Some(i.trim()) } else { None })
    {
        mm.update_rock_path(s);
    }

    let mut result = vec![];
    while let Some(p) = mm.run_once() {
        result.push(p);
    }
    result.len()
}

fn second(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(first(INPUT), 24);
    }
}
