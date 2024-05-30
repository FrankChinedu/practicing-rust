pub mod builder_pattern;
pub mod closure;
pub mod ds_and_as;
pub mod from;
pub mod myfs;
pub mod strings;
pub mod vec;

pub mod iter;
use ds_and_as::plus_one::Solution;

// use iter::main as run;

fn main() {
    let digits = vec![1, 2, 3];
    let res = Solution::plus_one(digits);

    dbg!(&res);
}
