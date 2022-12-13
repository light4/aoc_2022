use std::{cell::RefCell, cmp::Ordering, error::Error, fmt, rc::Rc, str::FromStr};

#[allow(dead_code)]
static INPUT: &str = r#"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"#;

pub fn run() {
    let input = include_str!("../input/day13/input");
    dbg!(first(input));
    dbg!(second(input));
}

pub struct Signal {
    left: Rc<RefCell<TreeNode>>,
    right: Rc<RefCell<TreeNode>>,
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Signal: [")?;
        writeln!(f, "    left: {}", self.left.as_ref().borrow())?;
        writeln!(f, "    right: {}", self.right.as_ref().borrow())?;
        writeln!(f, "]")?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Eq)]
struct TreeNode {
    pub level: u32,
    pub value: Option<u32>,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(level: u32) -> Self {
        Self {
            level,
            value: None,
            children: vec![],
            parent: None,
        }
    }

    #[inline]
    pub fn with_value(mut self, value: u32) -> Self {
        self.value = Some(value);
        self
    }

    #[inline]
    pub fn with_child(mut self, value: u32) -> Self {
        let child = Rc::new(RefCell::new(TreeNode::default().with_value(value)));
        self.children.push(child);
        self
    }

    #[allow(dead_code)]
    pub fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
        self.children.push(new_node);
    }
}

impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.value, other.value) {
            (Some(a), Some(b)) => a.cmp(&b),
            (Some(a), None) => {
                let new = TreeNode::default().with_child(a);
                new.cmp(other)
            }
            (None, Some(b)) => {
                let new = TreeNode::default().with_child(b);
                self.cmp(&new)
            }
            (None, None) => self.children.cmp(&other.children),
        }
    }
}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = self.value {
            write!(f, "{value}")?;
        } else {
            write!(f, "[")?;
            write!(
                f,
                "{}",
                self.children
                    .iter()
                    .map(|i| i.as_ref().borrow().to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            )?;
            write!(f, "]")?;
        }
        Ok(())
    }
}

fn init_tree(s: &str) -> Rc<RefCell<TreeNode>> {
    let root = Rc::new(RefCell::new(TreeNode::default()));
    let mut current = Rc::clone(&root);
    let chars = s.chars().collect::<Vec<char>>();
    let mut value_stack = vec![];
    let mut level = 0;
    let mut prev_char = ' ';
    for (_, c) in chars.iter().enumerate()
    // .filter(|(idx, _)| *idx > 0 && *idx + 1 < chars.len())
    {
        if c.is_numeric() && !value_stack.is_empty() {
            value_stack.push(c);
        } else if *c == '[' || c.is_numeric() {
            level += 1;
            let child = Rc::new(RefCell::new(TreeNode::new(level)));
            current.borrow_mut().children.push(Rc::clone(&child));
            {
                let mut mut_child = child.borrow_mut();
                mut_child.parent = Some(Rc::clone(&current));
                if c.is_numeric() {
                    value_stack.push(c);
                } else {
                    mut_child.value = (value_stack.iter().copied().collect::<String>())
                        .parse()
                        .ok();
                    value_stack.clear();
                }
            }
            current = child;
        } else if *c == ']' && prev_char == '[' {
            // dbg!();
        } else if *c == ',' || *c == ']' {
            level -= 1;
            let current_clone = Rc::clone(&current);
            {
                let mut mut_curr = current_clone.borrow_mut();
                if !value_stack.is_empty() {
                    mut_curr.value = (value_stack.iter().copied().collect::<String>())
                        .parse()
                        .ok();
                    value_stack.clear();
                }
            }
            current = Rc::clone(current_clone.as_ref().borrow().parent.as_ref().unwrap());
        } else {
            panic!("Unknown character: {c}");
        }
        prev_char = *c;
    }
    let temp_root = root.as_ref().borrow();
    temp_root.children[0].clone()
}

impl FromStr for Signal {
    type Err = ParseSignalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splited: Vec<&str> = s
            .split('\n')
            .map(|i| i.trim())
            .filter(|i| !i.is_empty())
            .collect();
        Ok(Self {
            left: init_tree(splited[0]),
            right: init_tree(splited[1]),
        })
    }
}

/// An error returned when parsing a `bool` using [`from_str`] fails
///
/// [`from_str`]: super::FromStr::from_str
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ParseSignalError;

impl fmt::Display for ParseSignalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "provided string was not a signal")
    }
}

impl Error for ParseSignalError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "failed to parse signal"
    }
}

fn first(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .enumerate()
        .filter_map(|(idx, s)| {
            let signal: Signal = s.parse().unwrap();
            let o = signal.left.cmp(&signal.right);
            assert_ne!(o, Ordering::Equal);
            if let Ordering::Less = o {
                Some(idx + 1)
            } else {
                None
            }
        })
        .sum()
}

fn second(input: &str) -> usize {
    let divider_packets_str = r#"
        [[2]]
        [[6]]
    "#;
    let mut tree_nodes: Vec<Rc<RefCell<TreeNode>>> = input
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(init_tree)
        .collect();
    let divider_packets: Vec<Rc<RefCell<TreeNode>>> = divider_packets_str
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(init_tree)
        .collect();
    tree_nodes.extend(divider_packets.clone());
    dbg!(&divider_packets.len());
    tree_nodes.sort();
    tree_nodes
        .iter()
        .enumerate()
        .filter_map(|(idx, s)| {
            if divider_packets.contains(s) {
                Some(idx + 1)
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_tree_1() {
        let tree = init_tree("[1,2]");
        assert_eq!(
            tree.as_ref().borrow().children[0]
                .as_ref()
                .borrow()
                .value
                .unwrap(),
            1
        );
    }

    #[test]
    fn test_init_tree_2() {
        let tree = init_tree("[1,2]");
        assert_eq!(
            tree.as_ref().borrow().children[1]
                .as_ref()
                .borrow()
                .value
                .unwrap(),
            2
        );
    }

    #[test]
    fn test_init_tree_3() {
        let s = "[0,1,[3,4,5,[7,8]],2]";
        let tree = init_tree(s);
        assert_eq!(tree.as_ref().borrow().to_string(), s);
    }

    #[test]
    fn test_init_tree_4() {
        let s = "[[7,1],[9,5,8,10],[[[5,1,0,10],[10,9,2,5]],4,5,2,2]]";
        let tree = init_tree(s);
        assert_eq!(tree.as_ref().borrow().to_string(), s);
    }

    #[test]
    fn test_init_tree_5() {
        let s = "[[8,10,[[8,5,8,6,10],[],[]],2,8],[]]";
        let tree = init_tree(s);
        assert_eq!(tree.as_ref().borrow().to_string(), s);
    }

    #[test]
    fn test_add_child() {
        let tree = init_tree("[0,1,[3,4,5,[7,8]],2]");
        let new_node = Rc::new(RefCell::new(TreeNode::default().with_value(9)));
        let child = &tree.as_ref().borrow().children[2];
        child.borrow_mut().add_child(new_node);
        assert_eq!(
            tree.as_ref().borrow().to_string(),
            "[0,1,[3,4,5,[7,8],9],2]"
        );
    }

    #[test]
    fn test_in_right_order_1() {
        let input = r#"
            [1,1,3,1,1]
            [1,1,5,1,1]
        "#;
        let s: Signal = input.parse().unwrap();
        assert!(s.left < s.right);
    }

    #[test]
    fn test_in_right_order_2() {
        let input = r#"
            [[1],[2,3,4]]
            [[1],4]
        "#;
        let s: Signal = input.parse().unwrap();
        assert!(s.left < s.right);
    }

    #[test]
    fn test_in_right_order_3() {
        let input = r#"
            [9]
            [[8,7,6]]
        "#;
        let s: Signal = input.parse().unwrap();
        assert!(s.left > s.right);
    }

    #[test]
    fn test_in_right_order_4() {
        let input = r#"
            [[4,4],4,4]
            [[4,4],4,4,4]
        "#;
        let s: Signal = input.parse().unwrap();
        assert!(s.left < s.right);
    }

    #[test]
    fn test_in_right_order_5() {
        let input = r#"
            [7,7,7,7]
            [7,7,7]
        "#;
        let s: Signal = input.parse().unwrap();
        assert!(s.left > s.right);
    }

    #[test]
    fn test_in_right_order_6() {
        let input = r#"
            []
            [3]
        "#;
        let s: Signal = input.parse().unwrap();
        assert!(s.left < s.right);
    }

    #[test]
    fn test_in_right_order_7() {
        let input = r#"
            [[[]]]
            [[]]
        "#;
        let s: Signal = input.parse().unwrap();
        assert!(s.left > s.right);
    }

    #[test]
    fn test_in_right_order_8() {
        let input = r#"
            [1,[2,[3,[4,[5,6,7]]]],8,9]
            [1,[2,[3,[4,[5,6,0]]]],8,9]
        "#;
        let s: Signal = input.parse().unwrap();
        assert!(s.left > s.right);
    }

    #[test]
    fn test_first() {
        assert_eq!(first(INPUT), 13);
    }

    #[test]
    fn test_second() {
        assert_eq!(second(INPUT), 140);
    }
}
