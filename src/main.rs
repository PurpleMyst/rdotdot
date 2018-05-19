#[macro_use]
extern crate nom;

extern crate reffers;

mod ast;
mod chain_map;
mod environment;
mod parser;
mod value;

fn run(code: &[u8]) -> reffers::rc::Strong<value::Value> {
    let ast = parser::program(code).unwrap().1;
    assert!(!ast.is_empty());
    let mut env = environment::Environment::prelude();
    //env.sub_environment();
    ast.into_iter()
        .filter(|node| match node {
            ast::AstNode::Comment(_) => false,
            _ => true,
        })
        .map(|node| env.eval(node))
        .last()
        .unwrap()
}

fn main() {
    let filename = std::env::args().nth(1).expect("USAGE: cargo run FILENAME");
    let program = std::fs::read(filename).unwrap();
    println!("{:?}", *run(&program).get());
}
