use std::{cell::RefCell, cmp::Ordering, error::Error, fmt, rc::Rc, str::FromStr};

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
    let input = r#"
            [[1],[2,3,4]]
            [[1],4]
        "#;
    let s: Signal = input.parse().unwrap();
    dbg!(s.is_in_right_order());
    // dbg!(test_order(&vec![1, 1, 3, 1, 1], &vec![1, 1, 5, 1, 1]));
    // dbg!(test_order(&vec![2, 3, 4], &vec![4]));
    // dbg!(test_order(&vec![9], &vec![8, 7, 6]));
    // dbg!(test_order(&vec![4, 4], &vec![4, 4, 4]));
    // dbg!(test_order(&vec![7, 7, 7], &vec![7, 7]));
    // dbg!(test_order(&vec![], &vec![3]));
    // dbg!(test_order(&vec![3], &vec![]));

    // for s in INPUT
    //     .split("\n\n")
    //     .map(|s| s.trim())
    //     .filter(|s| !s.is_empty())
    // {
    //     let signal: Signal = s.parse().unwrap();
    //     println!("{signal}");
    //     println!(
    //         "{:?}",
    //         is_vec_in_right_order(&[&signal.left], &[&signal.right])
    //     )
    //     // signal.is_in_right_order();
    // }
}

pub struct Signal {
    left: Rc<RefCell<TreeNode>>,
    right: Rc<RefCell<TreeNode>>,
}

impl Signal {
    pub fn is_in_right_order(&self) -> bool {
        is_in_right_order(&self.left.as_ref().borrow(), &self.right.as_ref().borrow()).unwrap()
    }
}

fn test_order(a: &[u32], b: &[u32]) -> bool {
    for (i, j) in a.iter().zip(b) {
        if i != j {
            return i < j;
        }
    }
    a.len() < b.len()
}

fn is_vec_in_right_order(
    left: &[&Rc<RefCell<TreeNode>>],
    right: &[&Rc<RefCell<TreeNode>>],
) -> Option<bool> {
    for (l, r) in left.iter().zip(right.iter()) {
        let left_node = l.as_ref().borrow();
        let right_node = r.as_ref().borrow();
        match (left_node.value, right_node.value) {
            (Some(a), Some(b)) => {
                if a != b {
                    return Some(a < b);
                } else {
                    continue;
                }
            }
            (Some(_), None) => {
                if right_node.children.is_empty() {
                    return Some(false);
                }
                return is_in_right_order(&left_node, &right_node.children[0].as_ref().borrow());
            }
            (None, Some(_)) => {
                if left_node.children.is_empty() {
                    return Some(true);
                }
                return is_in_right_order(&left_node.children[0].as_ref().borrow(), &right_node);
            }
            (None, None) => {
                return is_vec_in_right_order(
                    &left_node
                        .children
                        .iter()
                        .collect::<Vec<&Rc<RefCell<TreeNode>>>>(),
                    &right_node
                        .children
                        .iter()
                        .collect::<Vec<&Rc<RefCell<TreeNode>>>>(),
                );
            }
        }
    }
    None
}

fn is_in_right_order(left: &TreeNode, right: &TreeNode) -> Option<bool> {
    match (left.value, right.value) {
        (Some(a), Some(b)) => {
            if a != b {
                Some(a < b)
            } else {
                return is_vec_in_right_order(
                    &left
                        .children
                        .iter()
                        .collect::<Vec<&Rc<RefCell<TreeNode>>>>(),
                    &right
                        .children
                        .iter()
                        .collect::<Vec<&Rc<RefCell<TreeNode>>>>(),
                );
            }
        }
        (Some(_), None) => return is_in_right_order(left, &right.children[0].as_ref().borrow()),
        (None, Some(_)) => {
            return is_in_right_order(&left.children[0].as_ref().borrow(), right);
        }
        (None, None) => {
            for (l, r) in left.children.iter().zip(right.children.iter()) {
                if let Some(b) = is_in_right_order(&l.as_ref().borrow(), &r.as_ref().borrow()) {
                    return Some(b);
                }
            }
            match left.children.len().cmp(&right.children.len()) {
                Ordering::Less => Some(true),
                Ordering::Equal => None,
                Ordering::Greater => Some(false),
            }
        }
    }
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

#[derive(Default, PartialEq)]
struct TreeNode {
    pub value: Option<u32>,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new() -> Self {
        Self {
            value: None,
            children: vec![],
            parent: None,
        }
    }

    pub fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
        self.children.push(new_node);
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
    let root = Rc::new(RefCell::new(TreeNode::new()));
    let mut current = Rc::clone(&root);
    let chars = s.chars().collect::<Vec<char>>();
    let mut value_stack = vec![];
    for (_, c) in chars.iter().enumerate()
    // .filter(|(idx, _)| *idx > 0 && *idx + 1 < chars.len())
    {
        if c.is_numeric() && !value_stack.is_empty() {
            value_stack.push(c);
        } else if *c == '[' || c.is_numeric() {
            let child = Rc::new(RefCell::new(TreeNode::new()));
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
                }
            }
            current = child;
        } else if *c == ',' || *c == ']' {
            let current_clone = Rc::clone(&current);
            {
                let mut mut_child = current_clone.borrow_mut();
                if !value_stack.is_empty() {
                    mut_child.value = (value_stack.iter().copied().collect::<String>())
                        .parse()
                        .ok();
                    value_stack.clear();
                }
            }
            current = Rc::clone(current_clone.as_ref().borrow().parent.as_ref().unwrap());
        } else {
            panic!("Unknown character: {c}");
        }
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
    fn test_add_child() {
        let tree = init_tree("[0,1,[3,4,5,[7,8]],2]");
        let new_node = Rc::new(RefCell::new(TreeNode::new()));
        new_node.borrow_mut().value = Some(9);
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
        assert_eq!(s.is_in_right_order(), true);
    }

    #[test]
    fn test_in_right_order_2() {
        let input = r#"
            [[1],[2,3,4]]
            [[1],4]
        "#;
        let s: Signal = input.parse().unwrap();
        assert_eq!(s.is_in_right_order(), true);
    }

    #[test]
    fn test_in_right_order_3() {
        let input = r#"
            [9]
            [[8,7,6]]
        "#;
        let s: Signal = input.parse().unwrap();
        assert_eq!(s.is_in_right_order(), false);
    }

    #[test]
    fn test_in_right_order_4() {
        let input = r#"
            [[4,4],4,4]
            [[4,4],4,4,4]
        "#;
        let s: Signal = input.parse().unwrap();
        assert_eq!(s.is_in_right_order(), true);
    }

    #[test]
    fn test_in_right_order_5() {
        let input = r#"
            [7,7,7,7]
            [7,7,7]
        "#;
        let s: Signal = input.parse().unwrap();
        assert_eq!(s.is_in_right_order(), false);
    }

    #[test]
    fn test_in_right_order_6() {
        let input = r#"
            []
            [3]
        "#;
        let s: Signal = input.parse().unwrap();
        assert_eq!(s.is_in_right_order(), true);
    }

    #[test]
    fn test_in_right_order_7() {
        let input = r#"
            [[[]]]
            [[]]
        "#;
        let s: Signal = input.parse().unwrap();
        assert_eq!(s.is_in_right_order(), false);
    }

    #[test]
    fn test_in_right_order_8() {
        let input = r#"
            [1,[2,[3,[4,[5,6,7]]]],8,9]
            [1,[2,[3,[4,[5,6,0]]]],8,9]
        "#;
        let s: Signal = input.parse().unwrap();
        assert_eq!(s.is_in_right_order(), false);
    }
}
