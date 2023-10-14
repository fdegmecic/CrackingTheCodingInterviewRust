// 4.3 - List of Depths
#![allow(dead_code)]

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use super::q2::BinaryNode;

type Link = Rc<RefCell<BinaryNode>>;

fn list_of_depths(root: Option<Link>) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let mut queue = VecDeque::new();

    if let Some(root) = root {
        queue.push_back(root);
    }

    while !queue.is_empty() {
        let level_size = queue.len();
        let mut level_vals = Vec::new();

        for _ in 0..level_size {
            if let Some(node) = queue.pop_front() {
                let node = node.borrow();
                level_vals.push(node.value);

                if let Some(left) = node.left.clone() {
                    queue.push_back(left);
                }

                if let Some(right) = node.right.clone() {
                    queue.push_back(right);
                }
            }
        }

        result.push(level_vals);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::chapter4::q2::create_minimal_tree;

    use super::list_of_depths;

    #[test]
    fn should_list_depths() {
        let sorted_array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let tree = create_minimal_tree(&sorted_array);

        let actual = list_of_depths(tree);
        println!("{:?}", actual);
    }
}
