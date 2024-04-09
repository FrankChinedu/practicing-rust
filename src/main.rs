pub mod builder_pattern;
pub mod closure;
pub mod ds_and_as;
pub mod from;
pub mod myfs;
pub mod strings;
pub mod vec;

pub mod iter;
use ds_and_as::excel_sheet::Solution;

// use iter::main as run;

fn main() {
    let res = Solution::convert_to_title(703);

    dbg!(&res);
}
