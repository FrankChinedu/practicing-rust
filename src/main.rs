pub mod builder_pattern;
pub mod closure;
pub mod ds_and_as;
pub mod from;
pub mod myfs;
pub mod strings;
pub mod vec;

pub mod iter;
use ds_and_as::rotate_image::Solution;

fn main() {
    // let mut matrix = Vec::from([
    //     Vec::from([1, 2, 3]),
    //     Vec::from([4, 5, 6]),
    //     Vec::from([7, 8, 9]),
    // ]);
    let mut matrix = Vec::from([
        Vec::from([1, 2, 3, 4]),
        Vec::from([5, 6, 7, 8]),
        Vec::from([9, 10, 11, 12]),
        Vec::from([13, 14, 15, 16]),
    ]);
    Solution::rotate(&mut matrix);
    // dbg!(&res);
}
