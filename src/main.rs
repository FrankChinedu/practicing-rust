pub mod builder_pattern;
pub mod closure;
pub mod ds_and_as;
pub mod from;
pub mod myfs;
pub mod strings;
pub mod vec;

pub mod iter;
use ds_and_as::haystack::Solution;

// use iter::main as run;

fn main() {
    let haystack = String::from("leetcode");
    let needle = String::from("leeto");
    let res = Solution::str_str(haystack, needle);

    dbg!(&res);
}
