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
    let mut nums = Vec::from([1, 1, 2]);
    Solution::remove_duplicates(&mut nums);
    // dbg!(&res);
}
