use super::ast::AstNode;

use peeking_take_while::PeekableExt;

use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Token {
    StringLiteral(String),

    Identifier(String),

    IntegerLiteral(i64),

    LeftCurly,
    RightCurly,

    LeftBracket,
    RightBracket,

    LeftParenthesis,
    RightParenthesis,

    Equals,

    Semicolon,

    Dot,

    Backtick,

    DoubleColon,

    Var,
}

pub fn tokenize(code: &str) -> Vec<Token> {
    let mut result = Vec::new();
    let mut code_chars = code.chars().peekable();

    let mut is_negative = false;

    loop {
        let c: char = match code_chars.next() {
            Some(c) => c,
            None => break,
        };

        let token = match c {
            '#' => {
                code_chars
                    .by_ref()
                    .take_while(|&c| c != '\n')
                    .for_each(|_| {});
                continue;
            }

            '"' => {
                let contents = code_chars.by_ref().take_while(|&c| c != '"').collect();
                Token::StringLiteral(contents)
            }

            c if c.is_alphabetic() => {
                let contents = ::std::iter::once(c)
                    .chain(code_chars.by_ref().peeking_take_while(|&c| {
                        c.is_alphabetic() || c.is_ascii_digit() || c == '_'
                    }))
                    .collect();

                if contents == "var" {
                    Token::Var
                } else {
                    Token::Identifier(contents)
                }
            }

            '-' => {
                is_negative = true;
                continue;
            }

            c if c.is_ascii_digit() => {
                if c == '0'
                    && code_chars
                        .peek()
                        .map(|c| c.is_ascii_digit())
                        .unwrap_or(false)
                {
                    // TODO: Better error message + don't panic.
                    panic!("Tokenizing error.");
                }

                let mut num = ::std::iter::once(c)
                    .chain(
                        code_chars
                            .by_ref()
                            .peeking_take_while(|c| c.is_ascii_digit()),
                    )
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap();

                if is_negative {
                    num = -num;
                }
                is_negative = false;

                Token::IntegerLiteral(num)
            }

            c if c.is_whitespace() => continue,

            '{' => Token::LeftCurly,
            '}' => Token::RightCurly,

            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,

            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,

            '=' => Token::Equals,

            ';' => Token::Semicolon,

            '.' => Token::Dot,

            '`' => Token::Backtick,

            ':' => {
                if code_chars.next().map(|c| c != ':').unwrap_or(true) {
                    // TODO: Better error message + don't panic.
                    panic!("Tokenizing error while double coloning.");
                }

                Token::DoubleColon
            }

            c => unimplemented!("{:?}", c),
        };

        result.push(token);
    }

    debug_assert!(code_chars.next().is_none());
    result
}
