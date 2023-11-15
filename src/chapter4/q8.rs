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

    #[test]
    fn should_find_lca() {}
}
