use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display},
    str::FromStr,
};

pub fn run() {
    let input = include_str!("../input/day07/input").trim();
    dbg!(first(input));
    dbg!(second(input));
}

#[derive(Debug, Clone)]
pub struct Directory {
    level: usize,
    name: String,
    path: Vec<String>,
    files: HashMap<String, usize>,
    dirs: Vec<Directory>,
}

impl Directory {
    pub fn contains_dir(&self, name: &str) -> bool {
        self.dirs.iter().any(|i| i.name == name)
    }

    pub fn find_dir(&self, path: &[&str]) -> Option<&Directory> {
        if path.len() == 1 {
            if path[0] == self.name {
                return Some(self);
            } else {
                return None;
            }
        }
        for dir in &self.dirs {
            if dir.name == path[1] {
                return dir.find_dir(&path[1..]);
            }
        }
        None
    }

    pub fn find_dir_mut(&mut self, path: &[&str]) -> Option<&mut Directory> {
        if path.len() == 1 {
            if path[0] == self.name {
                return Some(self);
            } else {
                return None;
            }
        }
        for dir in &mut self.dirs {
            if dir.name == path[1] {
                return dir.find_dir_mut(&path[1..]);
            }
        }
        None
    }

    pub fn push_dir(&mut self, dir: Directory) {
        self.dirs.push(dir);
    }

    pub fn ensure_dir(&mut self, path: &[&str]) {
        if self.find_dir(path).is_none() {
            self.ensure_dir(&path[..(path.len() - 1)]);
            let parent = self.find_dir_mut(&path[..path.len() - 1]).unwrap();
            let new_dir = parent.new_dir(path.last().unwrap());
            parent.push_dir(new_dir);
        }
    }

    pub fn push_file(&mut self, name: &str, size: usize) {
        self.files.entry(name.to_string()).or_insert(size);
    }

    pub fn new_dir(&self, name: &str) -> Directory {
        let mut path = self.path.clone();
        path.push(name.to_string());
        Directory {
            level: path.len() - 1,
            name: name.to_string(),
            path,
            files: HashMap::new(),
            dirs: vec![],
        }
    }

    pub fn total_size(&self) -> usize {
        let direct_size: usize = self.files.values().sum();
        let sub_dirs_size: usize = self.dirs.iter().map(|i| i.total_size()).sum();
        direct_size + sub_dirs_size
    }

    fn total_size_vec(&self) -> Vec<usize> {
        let mut result = vec![];
        for dir in &self.dirs {
            let dir_total_size = dir.total_size();
            result.push(dir_total_size);
            result.extend(dir.total_size_vec());
        }
        result
    }
}

impl Default for Directory {
    fn default() -> Self {
        Self {
            level: 0,
            name: "/".to_string(),
            path: vec!["/".to_string()],
            files: HashMap::new(),
            dirs: vec![],
        }
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ident = " ".repeat(2 * self.level);
        f.write_str(&format!("{}- {} (dir)\n", ident, self.name))?;
        for (file, size) in &self.files {
            f.write_str(&format!("  {ident}- {file} (file, size={size})\n"))?;
        }
        for dir in &self.dirs {
            f.write_str(&format!("{dir}"))?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Cmd {
    CD(String),
    LS,
}

/// An error returned when parsing a `bool` using [`from_str`] fails
///
/// [`from_str`]: super::FromStr::from_str
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ParseChoiceError;

impl fmt::Display for ParseChoiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "provided string was not in allow list".fmt(f)
    }
}

impl Error for ParseChoiceError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "failed to parse cmd"
    }
}

impl FromStr for Cmd {
    type Err = ParseChoiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lst: Vec<&str> = s.split_whitespace().collect();
        match lst[1] {
            "ls" => Ok(Self::LS),
            "cd" => Ok(Self::CD(lst[2].to_string())),
            _ => Err(ParseChoiceError),
        }
    }
}

fn get_dirs(input: &str) -> Directory {
    fn find_next_cmd_idx(lines: &Vec<&str>, idx: usize) -> Option<usize> {
        for (i, item) in lines.iter().enumerate().skip(idx + 1) {
            if item.starts_with('$') {
                return Some(i);
            }
        }
        Some(lines.len())
    }

    let mut root = Directory::default();
    let mut current_path = root.path.clone();
    let lines: Vec<&str> = input
        .lines()
        .map(|l| l.trim())
        .filter(|i| !i.is_empty())
        .collect();
    for (idx, line) in lines.iter().enumerate() {
        if line.starts_with('$') {
            let cmd: Cmd = line.parse().unwrap();
            match cmd {
                Cmd::LS => {
                    let path: Vec<&str> = current_path.iter().map(|s| s.as_str()).collect();
                    let current_dir = root.find_dir_mut(&path).unwrap();
                    let next_cmd_idx = find_next_cmd_idx(&lines, idx).unwrap();
                    for entry in lines[(idx + 1)..next_cmd_idx].iter() {
                        let splited: Vec<&str> = entry.split_ascii_whitespace().collect();
                        if splited[0] == "dir" {
                            if !current_dir.contains_dir(splited[1]) {
                                current_dir.push_dir(current_dir.new_dir(splited[1]));
                            }
                        } else {
                            let (name, size) = (splited[1], splited[0].parse::<usize>().unwrap());
                            current_dir.files.entry(name.to_string()).or_insert(size);
                        }
                    }
                }
                Cmd::CD(to) => match to.as_str() {
                    "/" => {
                        current_path = root.path.clone();
                    }
                    ".." => {
                        current_path.pop();
                    }
                    name => {
                        current_path.push(name.to_string());
                    }
                },
            }
        }
    }

    root
}

fn first(input: &str) -> usize {
    let root = get_dirs(input);
    root.total_size_vec()
        .into_iter()
        .filter(|i| *i <= 100000)
        .sum()
}

fn second(input: &str) -> usize {
    let root = get_dirs(input);
    let mut total = root.total_size_vec();
    total.sort();
    let root_total = root.total_size();
    let unused = 70000000 - root_total;
    total
        .into_iter()
        .filter(|i| i + unused > 30000000)
        .take(1)
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    "#;
    #[test]
    fn test_first() {
        assert_eq!(first(INPUT), 95437);
    }

    #[test]
    fn test_second() {
        assert_eq!(second(INPUT), 24933642);
    }
}
