use std::rc::Rc;

pub fn check() {
    let some_large_test = "this is some very large test";
    let subsection: Rc<str> = Rc::from(&some_large_test[4..]);

    println!("subsection -> {subsection}");
    println!("some_large_test -> {some_large_test}")
}
