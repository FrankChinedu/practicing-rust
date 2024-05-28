pub mod builder_pattern;
pub mod closure;
pub mod ds_and_as;
pub mod from;
pub mod myfs;
pub mod strings;
pub mod vec;

pub mod iter;
use ds_and_as::search_insert_position::Solution;

// use iter::main as run;

fn main() {
    let nums = vec![1];
    let target = 0;
    // let nums = vec![1, 3, 6, 7];
    // let target = 5;
    let res = Solution::search_insert(nums, target);

    dbg!(&res);
}
