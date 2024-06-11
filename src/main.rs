pub mod builder_pattern;
pub mod closure;
pub mod ds_and_as;
pub mod from;
pub mod myfs;
pub mod strings;
pub mod vec;

pub mod iter;
use ds_and_as::merge_array::Solution;

fn main() {
    let mut num1 = Vec::from([-1, 0, 0, 3, 3, 3, 0, 0, 0]);
    let mut num2 = Vec::from([1, 2, 2]);
    let m = 6;
    let n = 3;
    Solution::merge(&mut num1, m, &mut num2, n);
    // dbg!(&res);
}
