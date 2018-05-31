extern crate peeking_take_while;

mod ast;
mod parser;

fn main() {
    let code = r#"
        # this is a comment
        "abcdef"
        identifier1 identifier2
        123 56 123abc
    "#;

    println!("{:#?}", parser::tokenize(code.trim()));
}
