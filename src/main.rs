#![deny(future_incompatible)]
#![warn(rust_2018_compatibility, rust_2018_idioms)]
extern crate peeking_take_while;
extern crate rpds;

pub mod ast;
pub mod parser;

fn main() {
    let code = std::fs::read_to_string(std::env::args().nth(1).expect("USAGE: cargo run FILENAME"))
        .expect("Error while reading");

    println!("{:#?}", parser::parse(&code).unwrap());
}
