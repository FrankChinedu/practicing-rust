pub mod builder_pattern;
pub mod closure;
pub mod ds_and_as;
pub mod from;
pub mod myfs;
pub mod strings;
pub mod vec;

pub mod iter;
use ds_and_as::binary_inorder_traversal::{get_tree_from_slice, Solution};

fn main() {
    let strs = Vec::from([2, 5, 1, 6, 7]);
    let res = get_tree_from_slice(strs);
    dbg!(&res);
}
