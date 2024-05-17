pub mod builder_pattern;
pub mod closure;
pub mod ds_and_as;
pub mod from;
pub mod myfs;
pub mod strings;
pub mod vec;

pub mod iter;
use ds_and_as::palindrome::Solution;

// use iter::main as run;

fn main() {
    let res = Solution::is_palindrome(-121);

    dbg!(&res);
}
