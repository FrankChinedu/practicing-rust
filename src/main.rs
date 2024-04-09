pub mod builder_pattern;
pub mod closure;
pub mod ds_and_as;
pub mod from;
pub mod myfs;
pub mod strings;
pub mod vec;

pub mod iter;
use ds_and_as::buy_tickets::Solution;

// use iter::main as run;

fn main() {
    let res = Solution::time_required_to_buy(vec![5, 1, 1, 1], 0);

    dbg!(&res);
}
