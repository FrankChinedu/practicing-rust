pub mod builder_pattern;
pub mod closure;
pub mod ds_and_as;
pub mod from;
pub mod myfs;
pub mod strings;
pub mod vec;

pub mod iter;
use ds_and_as::two_sum::Solution;

// use iter::main as run;

fn main() {
    let nums = vec![2, 11, 7, 15];
    let target = 9;

    let res = Solution::two_sum(nums, target);

    dbg!(&res);
}
