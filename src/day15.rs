use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
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
    // dbg!(first(input, 2000000));
    // dbg!(second(input));

    // let mm = init_map_with_empty_points(INPUT);
    // println!("{}", mm);
    dbg!(first(INPUT, 10));
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

    #[inline]
    pub fn manhattan_points(&self, dist: usize) -> Vec<Self> {
        let idist = dist as isize;
        let mut result = vec![];
        for x in (self.x - idist)..(self.x + idist) {
            for y in (self.y - idist)..(self.y + idist) {
                let pos = Position::new(x, y);
                if &pos != self && self.manhattan_distance(pos) <= dist {
                    result.push(pos)
                }
            }
        }
        result
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

    #[inline]
    fn get_row(&self, row: isize) -> HashMap<Position, Item> {
        self.items
            .iter()
            .filter(|(p, _)| p.y == row)
            .map(|(p, i)| (*p, *i))
            .collect()
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

fn init_map_with_empty_points(input: &str) -> MineMap {
    let mut mm = MineMap::default();
    let mut sb_dist_vec = vec![];
    let mut taken_points = HashSet::new();
    for s in input.lines().map(|i| i.trim()).filter(|i| !i.is_empty()) {
        let (sensor_pos, beacon_pos) = get_pos(s);
        mm.update_point(sensor_pos, Item::Sensor);
        mm.update_point(beacon_pos, Item::Beacon);
        sb_dist_vec.push((sensor_pos, sensor_pos.manhattan_distance(beacon_pos)));
        taken_points.insert(sensor_pos);
        taken_points.insert(beacon_pos);
    }
    for (s, d) in sb_dist_vec {
        let points = s.manhattan_points(d);
        for p in points {
            if !taken_points.contains(&p) {
                mm.update_point(p, Item::Empty);
            }
        }
    }
    mm
}

fn first(input: &str, row: isize) -> usize {
    let mm = init_map_with_empty_points(input);
    let row = mm.get_row(row);
    row.into_iter().filter(|(_, i)| *i == Item::Empty).count()
}

fn second(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(first(INPUT, 10), 26);
    }

    #[test]
    fn test_second() {
        assert_eq!(second(INPUT), 93);
    }
}
