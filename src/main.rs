pub mod builder_pattern;
pub mod closure;
pub mod ds_and_as;
pub mod from;
pub mod myfs;
pub mod strings;
pub mod vec;

pub mod iter;
use ds_and_as::sqrt::Solution;

// use iter::main as run;

fn main() {
    let x = 8;
    let res = Solution::my_sqrt(x);

    dbg!(&res);
}
