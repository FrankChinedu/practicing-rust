pub fn run() {
    let vec = vec![1, 2, 3, 4];

    let vec = &vec[0..2];

    display(vec);
}

fn display(vec: &[u8]) {
    println!("{:?}", vec)
}
