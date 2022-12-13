use std::{cell::RefCell, error::Error, fmt, rc::Rc, str::FromStr};

pub fn run() {
    todo!()
}

pub struct Signal {
    left: Rc<RefCell<TreeNode>>,
    right: Rc<RefCell<TreeNode>>,
}

#[derive(PartialEq)]
struct TreeNode {
    pub value: Option<u32>,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new() -> TreeNode {
        return TreeNode {
            value: None,
            children: vec![],
            parent: None,
        };
    }

    pub fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
        self.children.push(new_node);
    }
}

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = self.value {
            write!(f, "{}", value)?;
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
    for (_, c) in chars
        .iter()
        .enumerate()
        .filter(|(idx, _)| *idx > 0 && *idx + 1 < chars.len())
    {
        if *c == '[' || c.is_numeric() {
            let child = Rc::new(RefCell::new(TreeNode::new()));
            current.borrow_mut().children.push(Rc::clone(&child));
            {
                let mut mut_child = child.borrow_mut();
                mut_child.parent = Some(Rc::clone(&current));
                if c.is_numeric() {
                    mut_child.value = c.to_digit(10);
                }
            }
            current = child;
        } else if *c == ',' || *c == ']' {
            let current_clone = Rc::clone(&current);
            current = Rc::clone(current_clone.as_ref().borrow().parent.as_ref().unwrap());
        } else {
            panic!("Unknown character: {}", c);
        }
    }
    root
}

impl FromStr for Signal {
    type Err = ParseSignalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splited: Vec<&str> = s
            .split('\n')
            .map(|i| i.trim())
            .filter(|i| i.is_empty())
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
}
