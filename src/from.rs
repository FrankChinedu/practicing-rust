#[derive(Debug)]
pub struct UpperCase(pub String);

impl From<String> for UpperCase {
    fn from(value: String) -> Self {
        Self(value.to_uppercase())
    }
}

pub fn main() {
    let name = UpperCase::from(String::from("frank"));
    println!("{:?}", name.0);
}
