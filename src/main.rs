pub mod builder_pattern;
pub mod closure;
pub mod ds_and_as;
pub mod from;
pub mod myfs;
pub mod strings;
pub mod vec;

pub mod iter;
use ds_and_as::climb_stairs::Solution;

// use iter::main as run;

fn main() {
    let x = 4;
    let res = Solution::climb_stairs(x);

    dbg!(&res);
}
