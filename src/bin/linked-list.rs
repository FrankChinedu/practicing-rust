fn main() {
    let mut head = Node {
        value: 1,
        next: None,
    };

    let next = Node {
        next: None,
        value: 2,
    };

    head.set_next(next);

    println!("{:?}", head);
}

#[derive(Debug)]
#[allow(dead_code)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn set_next(&mut self, next: Node<T>) {
        self.next = Some(Box::new(next));
    }
}
