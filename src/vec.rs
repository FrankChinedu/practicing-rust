pub fn run() {
    let vec = vec![1, 2, 3, 4];

    display(&vec);
}

fn display(vec: &[u8]) {
    println!("{:?}", vec)
}
