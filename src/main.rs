pub mod builder_pattern;
pub mod closure;
pub mod ds_and_as;
pub mod from;
pub mod myfs;
pub mod strings;
pub mod vec;

pub mod iter;
use ds_and_as::remove_duplicates_array::Solution;

fn main() {
    let mut nums = Vec::from([0, 0, 1, 1, 1, 2, 2, 3, 3, 4]);
    Solution::remove_duplicates(&mut nums);
    // dbg!(&res);
}
