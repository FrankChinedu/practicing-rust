pub mod builder_pattern;
pub mod closure;
pub mod from;
pub mod myfs;
pub mod strings;
pub mod vec;

pub mod iter;

use iter::main as run;

fn main() {
    run();
}
