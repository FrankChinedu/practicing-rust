pub mod builder_pattern;
pub mod closure;
pub mod ds_and_as;
pub mod from;
pub mod myfs;
pub mod strings;
pub mod vec;

pub mod iter;
use ds_and_as::link_list_duplicate::{get_tree_from_slice, Solution};

fn main() {
    let head = get_tree_from_slice(Vec::from([1, 1, 1]));
    let _res = Solution::delete_duplicates(head);

    // dbg!(&res);
}
