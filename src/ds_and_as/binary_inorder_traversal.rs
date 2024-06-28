use std::borrow::BorrowMut;
// use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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

pub fn get_tree_from_slice(slice: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if slice.is_empty() {
        return None;
    }

    if slice.len() == 1 {
        if slice[0] == 0 {
            return None;
        }
        let tree = TreeNode::new(slice[0]);
        return Some(Rc::new(RefCell::new(tree)));
    }
    let first = *slice.first().unwrap();
    let mut tree = Rc::new(RefCell::new(TreeNode::new(first)));
    let arr_slice = &slice[1..];

    fn insert(root: &Rc<RefCell<TreeNode>>, arr_slice: &[i32]) {
        if arr_slice.is_empty() {
            return;
        }
        let val = arr_slice.first().unwrap();

        insert(root, &arr_slice[1..]);
    }
    insert(&mut tree, arr_slice);

    Some(tree)
}

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        [1].to_vec()
    }
}
