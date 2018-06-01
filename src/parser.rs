use super::ast::AstNode;

use peeking_take_while::PeekableExt;
use rpds::List;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
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

fn tokenize(code: &str) -> Vec<Token> {
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

#[derive(Debug, Clone)]
pub struct ParsingError(String);

impl From<String> for ParsingError { fn from(s: String) -> Self { ParsingError(s) } }
impl<'a> From<&'a str> for ParsingError { fn from(s: &'a str) -> Self { ParsingError(s.to_owned()) } }

fn function_call(mut tokens: List<Token>) -> Result<(List<Token>, AstNode), (List<Token>, ParsingError)> {
    let original_tokens = tokens.clone();

    let mut result = Vec::new();
    let last_error;

    loop {
        match expression(tokens) {
            Ok((new_tokens, node)) => {
                tokens = new_tokens;
                result.push(node)
            }

            Err((new_tokens, e)) => {
                tokens = new_tokens;
                last_error = e;
                break;
            }
        }
    }

    if result.len() == 0 { return Err((original_tokens, ParsingError::from(format!("Empty function call expression ( last argument error {} )", last_error.0)))) }

    let func = Box::new(result.drain(0..1).next().unwrap());

    Ok((tokens, AstNode::FunctionCall { func, args: result }))
}

fn expression(mut tokens: List<Token>) -> Result<(List<Token>, AstNode), (List<Token>, ParsingError)> {
    let node = match tokens.first().map(ToOwned::to_owned) {
        Some(Token::StringLiteral(s)) => {
            tokens.drop_first_mut();
            AstNode::StringLiteral(s)
        }

        Some(Token::IntegerLiteral(n)) => {
            tokens.drop_first_mut();
            AstNode::IntegerLiteral(n)
        }

        Some(Token::Identifier(s)) => { tokens.drop_first_mut(); AstNode::VariableLookup(s) },

        Some(Token::LeftParenthesis) => {
            let original_tokens = tokens.clone();
            tokens.drop_first_mut();

            match function_call(tokens) {
                Ok((mut new_tokens, node)) => {
                    if !new_tokens.drop_first_mut()
                        || new_tokens.first().cloned() != Some(Token::RightParenthesis)
                    {
                        return Err((original_tokens, ParsingError::from("Missing right parenthesis.")));
                    } else {
                        assert!(new_tokens.drop_first_mut());
                    }
                    tokens = new_tokens;
                    node
                }

                Err((_, e)) => return Err((original_tokens, e)),
            }
        }

        // FIXME Support expression blocks.
        Some(Token::LeftCurly) => {
            let original_tokens = tokens.clone();
            tokens.drop_first_mut();

            let mut result = Vec::new();

            loop {
                match statement(tokens) {
                    Ok((new_tokens, node)) => {
                        result.push(node);
                        tokens = new_tokens;
                    }

                    Err((new_tokens, _)) => {
                        tokens = new_tokens;
                        break;
                    }
                }
            }

            if tokens.first().cloned() != Some(Token::RightCurly) {
                return Err((original_tokens, ParsingError::from("missing right curly.")));
            }
            tokens.drop_first_mut();

            AstNode::BlockStatement(result)
        }

        t => return Err((tokens, ParsingError::from(format!("Unexpected token {:?}", t)))),
    };

    Ok((tokens, node))
}

fn statement(tokens: List<Token>) -> Result<(List<Token>, AstNode), (List<Token>, ParsingError)> {
    let original_tokens = tokens.clone();

    let (mut tokens, node) = function_call(tokens)?;

    if tokens.first().cloned() != Some(Token::Semicolon) {
        return Err((original_tokens, ParsingError::from("Missing semicolon.")));
    }

    tokens.drop_first_mut();

    Ok((tokens, node))
}

pub fn parse(code: &str) -> Result<Vec<AstNode>, ParsingError> {
    let mut tokens = tokenize(code).into_iter().collect();
    let mut result = Vec::new();

    loop {
        match statement(tokens) {
            Ok((new_tokens, node)) => {
                result.push(node);
                tokens = new_tokens;
            }

            Err((_new_tokens, e)) => {
                if result.is_empty() {
                    return Err(e);
                }

                break;
            }
        }
    }

    Ok(result)
}
