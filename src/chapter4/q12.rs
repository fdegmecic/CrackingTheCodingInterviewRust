// 4.12 - Paths with Sum
#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

type Link = Rc<RefCell<BinaryNode>>;

struct BinaryNode {
    value: isize,
    left: Option<Link>,
    right: Option<Link>,
}

fn paths_with_sum(tree: &Option<Link>, sum: isize) -> usize {
    let paths = Vec::new();
    r_paths_with_sum(&tree, &paths, sum)
}

fn r_paths_with_sum(node: &Option<Link>, paths: &[Vec<isize>], sum: isize) -> usize {
    let node = match node {
        Some(node) => node,
        None => return 0,
    };
    let value = node.borrow().value;

    let matches = paths
        .iter()
        .filter(|path| {
            let s: isize = path.iter().sum();
            s + value == sum
        })
        .count();

    let mut child_paths: Vec<Vec<isize>> = paths
        .iter()
        .map(|s| {
            let mut s = s.to_vec();
            s.push(value);
            s
        })
        .collect();

    child_paths.push(vec![value]);

    matches
        + r_paths_with_sum(&node.borrow().left, &child_paths, sum)
        + r_paths_with_sum(&node.borrow().right, &child_paths, sum)
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::chapter4::q12::paths_with_sum;

    use super::{BinaryNode, Link};

    #[test]
    fn should_return_paths_with_sum() {
        let tree = get_tree();

        assert_eq!(paths_with_sum(&Some(tree), 8), 2);
    }

    fn get_tree() -> Link {
        let leaf_3 = Rc::new(RefCell::new(BinaryNode {
            value: 3,
            left: None,
            right: None,
        }));

        let leaf_2 = Rc::new(RefCell::new(BinaryNode {
            value: 2,
            left: None,
            right: None,
        }));

        let leaf_11 = Rc::new(RefCell::new(BinaryNode {
            value: 11,
            left: None,
            right: None,
        }));

        let node_5 = Rc::new(RefCell::new(BinaryNode {
            value: 5,
            left: Some(leaf_3.clone()),
            right: Some(leaf_2.clone()),
        }));

        let node_0 = Rc::new(RefCell::new(BinaryNode {
            value: -3,
            left: None,
            right: Some(leaf_11.clone()),
        }));

        Rc::new(RefCell::new(BinaryNode {
            value: 10,
            left: Some(node_5.clone()),
            right: Some(node_0.clone()),
        }))
    }
}
