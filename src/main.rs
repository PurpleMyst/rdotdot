#[macro_use]
extern crate nom;

extern crate reffers;

mod ast;
mod chain_map;
mod parser;
mod scope;
mod value;

fn run(code: &[u8]) -> reffers::rc::Strong<value::Value> {
    let ast = parser::program(code).unwrap().1;
    assert!(!ast.is_empty());
    let mut scope = scope::Scope::new();
    ast.into_iter()
        .filter(|node| match node {
            ast::AstNode::Comment(_) => false,
            _ => true,
        })
        .map(|node| scope.eval(node))
        .last()
        .unwrap()
}

fn main() {
    let filename = std::env::args().nth(1).expect("USAGE: cargo run FILENAME");
    let program = std::fs::read(filename).unwrap();
    println!("{:?}", *run(&program).get());
}
