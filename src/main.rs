pub mod builder_pattern;
pub mod closure;
pub mod ds_and_as;
pub mod from;
pub mod myfs;
pub mod strings;
pub mod vec;

pub mod iter;
use ds_and_as::matrix_zero::Solution;

fn main() {
    let mut matrix = Vec::from([
        Vec::from([1, 1, 1]),
        Vec::from([1, 0, 1]),
        Vec::from([1, 1, 1]),
    ]);
    Solution::set_zeroes(&mut matrix);
    // dbg!(&res);
}
