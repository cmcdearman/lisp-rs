use std::fs;

fn main() {
    let contents = fs::read("simple.eli")
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", String::from_utf8(contents).unwrap());
}
