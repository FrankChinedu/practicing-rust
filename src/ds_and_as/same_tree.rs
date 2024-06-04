use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn get_tree_from_slice(slice: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if slice.len() == 1 {
        let tree = TreeNode::new(slice[0]);
        return Some(Rc::new(RefCell::new(tree)));
    }
    let first = *slice.first().unwrap();
    let mut tree: TreeNode = TreeNode::new(first);
    let arr_slice = &slice[1..];
    for (i, val) in arr_slice.iter().enumerate() {
        let sub_tree = Rc::new(RefCell::new(TreeNode::new(*val)));
        if i % 2 == 0 {
            let left = tree.left.borrow_mut();
            left.replace(sub_tree);
        } else {
            let right = tree.right.borrow_mut();
            right.replace(sub_tree);
        }
    }

    Some(Rc::new(RefCell::new(tree)))
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_column_number_1() {
        let p = get_tree_from_slice(Vec::from([1, 2, 3]));
        let q = get_tree_from_slice(Vec::from([1, 2, 3]));
        let result = Solution::is_same_tree(p, q);
        assert!(result);
    }

    #[test]
    fn simple_column_number_2() {
        let p = get_tree_from_slice(Vec::from([1, 2]));
        let q = get_tree_from_slice(Vec::from([1, 0, 2]));
        let result = Solution::is_same_tree(p, q);
        assert!(!result);
    }

    #[test]
    fn simple_column_number_3() {
        let p = get_tree_from_slice(Vec::from([1, 2, 1]));
        let q = get_tree_from_slice(Vec::from([1, 1, 2]));
        let result = Solution::is_same_tree(p, q);
        assert!(!result);
    }
}
