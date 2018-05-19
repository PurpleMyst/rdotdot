#[macro_use]
extern crate nom;

mod ast;
mod parser;

fn main() {
    let filename = std::env::args().nth(1).expect("USAGE: cargo run FILENAME");
    let program = std::fs::read(filename).unwrap();
    let ast = parser::program(&program).unwrap().1;
    assert!(!ast.is_empty());
    println!("{:#?}", ast);
}
