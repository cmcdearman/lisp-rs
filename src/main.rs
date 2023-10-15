mod analysis;
mod compile;
mod syntax;
mod util;
mod vm;

fn main() {
    env_logger::init();
    let src = "(+ 1 2)";
    let (root, errs) = syntax::reader::read::read(src);
    if !errs.is_empty() {
        panic!("{:?}", errs);
    }
    println!("sexprs: {:?}", root.clone().unwrap());
    match syntax::parser::parse::parse(src, &root.unwrap()) {
        Ok(ast) => println!("ast: {:?}", ast),
        Err(e) => panic!("{:?}", e),
    }
}
