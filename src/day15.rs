use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{self, Debug},
    ops::Deref,
};

#[allow(dead_code)]
static INPUT: &str = r#"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#;

pub fn run() {
    // let input = include_str!("../input/day15/input");
    // dbg!(first(input));
    // dbg!(second(input));

    let mm = init_map(INPUT);
    dbg!(&mm);
    println!("{}", &mm);
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    x: isize,
    y: isize,
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
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn manhattan_distance(&self, other: Self) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    #[default]
    Unknow,
    Beacon,
    Sensor,
    Empty,
}

#[derive(Debug, Clone, Default)]
pub struct MineMap {
    items: HashMap<Position, Item>,
    edge: Edge,
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Edge {
    left: isize,
    right: isize,
    top: isize,
    down: isize,
}

impl Edge {
    #[inline]
    pub fn expand(&self, context: isize) -> Self {
        Self {
            left: (self.left - context),
            right: (self.right + context),
            top: (self.top - context),
            down: (self.down + context),
        }
    }

    #[inline]
    pub fn grow(&mut self, pos: &Position) {
        self.left = self.left.min(pos.x);
        self.right = self.right.max(pos.x);
        self.top = self.top.min(pos.y);
        self.down = self.down.max(pos.y);
    }
}

impl MineMap {
    #[inline]
    fn update_point(&mut self, pos: impl Into<Position>, item: Item) {
        let pos = pos.into();
        self.edge.grow(&pos);
        self.items.insert(pos, item);
    }

    #[inline]
    fn pos_item_mut(&mut self, pos: impl Into<Position>) -> Option<&mut Item> {
        let p = pos.into();
        self.items.get_mut(&p)
    }
}

impl Deref for MineMap {
    type Target = HashMap<Position, Item>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl fmt::Display for MineMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let edge = self.edge.expand(3);
        writeln!(f, "edge: {:?}", edge)?;
        for col in edge.top..=edge.down {
            write!(f, "{col:3} ")?;
            for row in edge.left..=edge.right {
                let pos = Position::new(row, col);
                if let Some(item) = self.get(&pos) {
                    match item {
                        Item::Beacon => write!(f, "B")?,
                        Item::Empty => write!(f, "#")?,
                        Item::Sensor => write!(f, "S")?,
                        Item::Unknow => write!(f, ".")?,
                    };
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn get_pos(s: &str) -> (Position, Position) {
    let result: Vec<Position> = s
        .split(':')
        .map(|i| {
            let splited: Vec<isize> = i
                .split(',')
                .map(|j| j.split('=').last().unwrap().parse::<isize>().unwrap())
                .collect();
            Position::new(splited[0], splited[1])
        })
        .collect();
    (result[0], result[1])
}

fn init_map(input: &str) -> MineMap {
    let mut mm = MineMap::default();
    for s in input.lines().map(|i| i.trim()).filter(|i| !i.is_empty()) {
        let (sensor_pos, beacon_pos) = get_pos(s);
        mm.update_point(sensor_pos, Item::Sensor);
        mm.update_point(beacon_pos, Item::Beacon);
    }
    mm
}

fn first(input: &str) -> usize {
    todo!()
}

fn second(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(first(INPUT), 26);
    }

    #[test]
    fn test_second() {
        assert_eq!(second(INPUT), 93);
    }
}
