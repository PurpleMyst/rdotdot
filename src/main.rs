#![warn(
    anonymous_parameters, bare_trait_object, missing_copy_implementations,
    missing_debug_implementations, trivial_casts, trivial_numeric_casts, unreachable_pub,
    unsafe_code, unstable_features, unused_extern_crates, unused_import_braces,
    unused_qualifications, unused_results, variant_size_differences
)]
#![deny(future_incompatible)]
extern crate peeking_take_while;
extern crate rpds;

pub mod ast;
pub mod parser;

fn main() {
    let code = std::fs::read_to_string(std::env::args().nth(1).expect("USAGE: cargo run FILENAME"))
        .expect("Error while reading");

    println!("{:#?}", parser::parse(&code).unwrap());
}
