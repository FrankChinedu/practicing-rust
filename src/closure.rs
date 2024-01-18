fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b)
}

pub fn main() {
    let add = Box::new(|a, b| a + b);

    let res = math(1, 2, add);
    println!("{res}")
}
