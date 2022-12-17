use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt,
    ops::Deref,
};

pub fn run() {
    let mut mm = MineMap::new(7);
    dbg!(&mm);
    println!("{}", &mm);
    mm.add_rock(Rock::Squar);
    println!("{}", &mm);
    // first(INPUT, 2022);
}

// ####
//
// .#.
// ###
// .#.
//
// ..#
// ..#
// ###
//
// #
// #
// #
// #
//
// ##
// ##
#[derive(Debug, Clone, Copy)]
pub enum Rock {
    Minus,
    Plus,
    LFlip,
    VertLine,
    Squar,
}

#[derive(Debug, Clone)]
pub struct RockItem {
    rock: Rock,
    pos: HashSet<Position>,
}

impl RockItem {
    pub fn new(rock: Rock) -> Self {
        let mut pos = HashSet::new();
        match rock {
            Rock::Minus => {
                for i in 0..4 {
                    pos.insert(Position::new(i, 0));
                }
            }
            Rock::Plus => {
                for i in 0..3 {
                    pos.insert(Position::new(1, i));
                    pos.insert(Position::new(i, 1));
                }
            }
            Rock::LFlip => {
                for i in 0..3 {
                    pos.insert(Position::new(i, 0));
                    pos.insert(Position::new(2, i));
                }
            }
            Rock::VertLine => {
                for i in 0..4 {
                    pos.insert(Position::new(0, i));
                }
            }
            Rock::Squar => {
                for i in 0..2 {
                    pos.insert(Position::new(0, i));
                    pos.insert(Position::new(1, i));
                }
            }
        }
        Self { rock, pos }
    }
}

const ALL_ROCKS: [Rock; 5] = [
    Rock::Minus,
    Rock::Plus,
    Rock::LFlip,
    Rock::VertLine,
    Rock::Squar,
];

static INPUT: &str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    #[default]
    Empty,
    Wall,
    Floor,
    Corner,
    Rock,
    RestRock,
}

#[derive(Debug, Clone, Default)]
pub struct MineMap {
    items: HashMap<Position, Item>,
    edge: Edge,
    rock_item: Option<RockItem>,
    highest: usize,
}

impl MineMap {
    pub fn new(wide: usize) -> Self {
        let edge = Edge {
            left: 0,
            right: wide + 1,
            top: wide + 1,
            down: 0,
        };

        let mut items = HashMap::new();
        items.insert(Position::new(0, 0), Item::Corner);
        items.insert(Position::new(wide + 1, 0), Item::Corner);
        for i in 1..=edge.top {
            items.insert(Position::new(0, i), Item::Wall);
            items.insert(Position::new(edge.right, i), Item::Wall);
        }
        for i in 1..edge.right {
            items.insert(Position::new(i, 0), Item::Floor);
        }
        Self {
            edge: Edge {
                left: 0,
                right: wide + 1,
                top: wide + 1,
                down: 0,
            },
            items,
            ..Default::default()
        }
    }

    pub fn add_rock(&mut self, rock: Rock) {
        assert!(self.rock_item.is_none());
        let mut item = RockItem::new(rock);
        let new_pos = item
            .pos
            .iter()
            .map(|p| Position::new(p.x + 3, p.y + self.highest + 4))
            .collect();
        item.pos = new_pos;
        self.rock_item = Some(item);
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
        writeln!(f, "edge: {:?}", self.edge)?;
        for col in (self.edge.down..=self.edge.top).rev() {
            write!(f, "{col:4} ")?;
            for row in self.edge.left..=self.edge.right {
                let pos = Position::new(row, col);

                if let Some(item) = self.get(&pos) {
                    match item {
                        Item::Corner => write!(f, "+")?,
                        Item::Wall => write!(f, "|")?,
                        Item::Floor => write!(f, "_")?,
                        Item::Rock => write!(f, "@")?,
                        Item::RestRock => write!(f, "#")?,
                        Item::Empty => write!(f, ".")?,
                    };
                } else if let Some(rock_item) = &self.rock_item {
                    if rock_item.pos.contains(&pos) {
                        write!(f, "@")?;
                    } else {
                        write!(f, ".")?;
                    }
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Edge {
    left: usize,
    right: usize,
    top: usize,
    down: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
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

fn first(input: &str, rocks: usize) -> usize {
    for rock in std::iter::repeat(ALL_ROCKS).flatten().take(rocks) {
        dbg!(rock);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(first(INPUT, 2022), 3068);
    }
}
