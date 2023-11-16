// 4.8 - First Common Ancestor
#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

use super::q2::BinaryNode;

type TreeNode = Rc<RefCell<BinaryNode>>;

fn lowest_common_ancestor(root: Option<TreeNode>, p: usize, q: usize) -> Option<usize> {
    r_lca(&root, p, q).map(|value| value.borrow().value)
}

fn r_lca(root: &Option<TreeNode>, p: usize, q: usize) -> Option<TreeNode> {
    if let Some(node) = root {
        if node.borrow().value == p || node.borrow().value == q {
            return root.clone();
        }

        let left = r_lca(&node.borrow().left, p, q);
        let right = r_lca(&node.borrow().right, p, q);

        match (&left, &right) {
            (None, None) => None,
            (None, _) => right,
            (_, None) => left,
            _ => root.clone(),
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::chapter4::q2::BinaryNode;

    use super::{lowest_common_ancestor, TreeNode};

    #[test]
    fn should_find_lca() {
        //       2
        //      /\
        //     10 30
        //    / \
        //   55  15
        //  /  \   \
        // 3   37   17

        let tree = get_tree();

        let result = lowest_common_ancestor(Some(tree), 37, 17);
        assert_eq!(result.unwrap(), 10);
    }

    fn get_tree() -> TreeNode {
        let leaf_1 = Rc::new(RefCell::new(BinaryNode {
            value: 3,
            left: None,
            right: None,
        }));
        let leaf_2 = Rc::new(RefCell::new(BinaryNode {
            value: 37,
            left: None,
            right: None,
        }));
        let leaf_3 = Rc::new(RefCell::new(BinaryNode {
            value: 17,
            left: None,
            right: None,
        }));
        let leaf_4 = Rc::new(RefCell::new(BinaryNode {
            value: 55,
            left: Some(leaf_1.clone()),
            right: Some(leaf_2.clone()),
        }));
        let leaf_5 = Rc::new(RefCell::new(BinaryNode {
            value: 15,
            left: None,
            right: Some(leaf_3.clone()),
        }));
        let leaf_6 = Rc::new(RefCell::new(BinaryNode {
            value: 10,
            left: Some(leaf_4.clone()),
            right: Some(leaf_5.clone()),
        }));
        let leaf_7 = Rc::new(RefCell::new(BinaryNode {
            value: 30,
            left: None,
            right: None,
        }));
        let root = Rc::new(RefCell::new(BinaryNode {
            value: 4,
            left: Some(leaf_6.clone()),
            right: Some(leaf_7.clone()),
        }));

        root
    }
}
