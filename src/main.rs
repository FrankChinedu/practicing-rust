pub mod builder_pattern;
pub mod from;
pub mod myfs;
pub mod vec;

use from::UpperCase;

fn main() {
    let name = UpperCase::from(String::from("frank"));
    println!("{:?}", name.0);
}
