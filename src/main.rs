pub mod builder_pattern;
pub mod closure;
pub mod ds_and_as;
pub mod from;
pub mod myfs;
pub mod strings;
pub mod vec;

pub mod iter;
use ds_and_as::same_tree::{get_tree_from_slice, Solution};

fn main() {
    let p = get_tree_from_slice(Vec::from([1, 2, 3]));
    let q = get_tree_from_slice(Vec::from([1, 2, 3]));
    let res = Solution::is_same_tree(p, q);

    dbg!(&res);
}
