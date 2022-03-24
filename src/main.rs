use std::fs;
// use crate::token;

fn main() {
    let contents = fs::read("examples/simple.el")
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", String::from_utf8(contents).unwrap());
}
