pub mod builder_pattern;
pub mod closure;
pub mod ds_and_as;
pub mod from;
pub mod myfs;
pub mod strings;
pub mod vec;

pub mod iter;
use ds_and_as::add_binary::Solution;

// use iter::main as run;

fn main() {
    let a = "1010".to_string();
    let b = "1011".to_string();
    let res = Solution::add_binary(a, b);

    dbg!(&res);
}
