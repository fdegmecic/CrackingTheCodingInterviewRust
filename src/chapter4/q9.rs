// 4.9 - BST Sequences
#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

type Link = Rc<RefCell<BinaryNode>>;

struct BinaryNode {
    value: usize,
    left: Option<Link>,
    right: Option<Link>,
}

fn bst_sequences(tree: Option<Link>) -> Vec<Vec<usize>> {
    r_bst_sequences(tree)
}

fn r_bst_sequences(node: Option<Link>) -> Vec<Vec<usize>> {
    let mut result = Vec::new();

    if let Some(node) = node {
        let prefix = node.borrow().value;

        let left_sequences = bst_sequences(node.borrow().left.clone());
        let right_sequences = bst_sequences(node.borrow().right.clone());

        for left in &left_sequences {
            for right in &right_sequences {
                let weaved_sequences = weave(left, right);
                for mut weave in weaved_sequences {
                    weave.insert(0, prefix);
                    result.push(weave);
                }
            }
        }
        return result;
    }
    result.push(Vec::new());
    result
}

fn weave(left: &[usize], right: &[usize]) -> Vec<Vec<usize>> {
    let mut temp: Vec<usize> = Vec::new();
    let mut result: Vec<Vec<usize>> = Vec::new();
    r_weave(
        &mut Vec::from(left.clone()),
        &mut Vec::from(right.clone()),
        temp.as_mut(),
        result.as_mut(),
    );

    result
}

fn r_weave(
    left: &mut Vec<usize>,
    right: &mut Vec<usize>,
    temp: &mut Vec<usize>,
    result: &mut Vec<Vec<usize>>,
) {
    if left.is_empty() || right.is_empty() {
        let mut results = temp.clone();
        results.append(&mut left.clone());
        results.append(&mut right.clone());
        result.push(results);
        return;
    }

    let first_left = left.remove(0);
    temp.push(first_left);
    r_weave(left, right, temp, result);
    temp.pop();
    left.insert(0, first_left);

    let first_right = right.remove(0);
    temp.push(first_right);
    r_weave(left, right, temp, result);
    temp.pop();
    right.insert(0, first_right);
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::{bst_sequences, BinaryNode, Link};

    #[test]
    fn should_get_all_permutations() {
        let tree = get_tree();
        let result = bst_sequences(Some(tree));

        assert_eq!(vec![2, 1, 3], result[0]);
        assert_eq!(vec![2, 3, 1], result[1]);
    }

    fn get_tree() -> Link {
        //    2
        //   / \
        //  1   3

        let leaf_1 = Rc::new(RefCell::new(BinaryNode {
            value: 1,
            left: None,
            right: None,
        }));
        let leaf_2 = Rc::new(RefCell::new(BinaryNode {
            value: 3,
            left: None,
            right: None,
        }));
        let root = Rc::new(RefCell::new(BinaryNode {
            value: 2,
            left: Some(leaf_1),
            right: Some(leaf_2),
        }));

        root
    }
}
