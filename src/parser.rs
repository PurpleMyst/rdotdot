// TODO:
//
// Expressions:
//    Infix function calls
//
// Misc:
//    Better errors!
//    Remove a few `.cloned()`
//    Refactor this code. Heavily.

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

            c => unimplemented!("{:?}", c),
        };

        result.push(token);
    }

    debug_assert!(code_chars.next().is_none());
    result
}

#[derive(Debug, Clone)]
pub struct ParsingError(String);

impl From<String> for ParsingError {
    fn from(s: String) -> Self {
        ParsingError(s)
    }
}
impl<'a> From<&'a str> for ParsingError {
    fn from(s: &'a str) -> Self {
        ParsingError(s.to_owned())
    }
}

fn function_call(
    mut tokens: List<Token>,
) -> Result<(List<Token>, AstNode), (List<Token>, ParsingError)> {
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

    if result.len() == 0 {
        return Err((
            original_tokens,
            ParsingError::from(format!(
                "Empty function call expression ( last argument error {} )",
                last_error.0
            )),
        ));
    }

    let func = Box::new(result.drain(0..1).next().unwrap());

    Ok((tokens, AstNode::FunctionCall { func, args: result }))
}

fn expression(
    mut tokens: List<Token>,
) -> Result<(List<Token>, AstNode), (List<Token>, ParsingError)> {
    let node = match tokens.first().map(ToOwned::to_owned) {
        Some(Token::StringLiteral(s)) => {
            tokens.drop_first_mut();
            AstNode::StringLiteral(s)
        }

        Some(Token::IntegerLiteral(n)) => {
            tokens.drop_first_mut();
            AstNode::IntegerLiteral(n)
        }

        Some(Token::Identifier(s)) => {
            tokens.drop_first_mut();
            AstNode::VariableLookup(s)
        }

        Some(Token::LeftParenthesis) => {
            let original_tokens = tokens.clone();
            tokens.drop_first_mut();

            match function_call(tokens) {
                Ok((mut new_tokens, node)) => {
                    if new_tokens.first().cloned() != Some(Token::RightParenthesis) {
                        return Err((
                            original_tokens,
                            ParsingError::from("Missing right parenthesis."),
                        ));
                    } else {
                        assert!(new_tokens.drop_first_mut());
                    }
                    tokens = new_tokens;
                    node
                }

                Err((_, e)) => return Err((original_tokens, e)),
            }
        }

        Some(Token::LeftBracket) => {
            let original_tokens = tokens.clone();
            tokens.drop_first_mut();
            let mut contents = Vec::new();

            loop {
                match expression(tokens) {
                    Ok((new_tokens, node)) => {
                        tokens = new_tokens;
                        contents.push(node)
                    }

                    Err((new_tokens, _)) => {
                        tokens = new_tokens;
                        break;
                    }
                }
            }

            if tokens.first().cloned() != Some(Token::RightBracket) {
                return Err((
                    original_tokens,
                    ParsingError::from("Missing right bracket."),
                ));
            } else {
                assert!(tokens.drop_first_mut());
            }

            AstNode::List(contents)
        }

        Some(Token::LeftCurly) => {
            let original_tokens = tokens.clone();
            tokens.drop_first_mut();

            let node = {
                {
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

                    if result.is_empty() {
                        match expression(tokens) {
                            Ok((new_tokens, expr)) => {
                                tokens = new_tokens;
                                AstNode::ExpressionBlock(Box::new(expr))
                            }

                            Err((new_tokens, _)) => {
                                tokens = new_tokens;
                                AstNode::StatementBlock(result)
                            }
                        }
                    } else {
                        AstNode::StatementBlock(result)
                    }
                }
            };

            if tokens.first().cloned() != Some(Token::RightCurly) {
                return Err((original_tokens, ParsingError::from("missing right curly.")));
            }
            assert!(tokens.drop_first_mut());

            node
        }

        t => {
            return Err((
                tokens,
                ParsingError::from(format!("Unexpected token {:?}", t)),
            ))
        }
    };

    let original_tokens = tokens.clone();

    if tokens.first().cloned() == Some(Token::Dot) {
        let mut attrs = Vec::new();

        while tokens.first() == Some(&Token::Dot) {
            assert!(tokens.drop_first_mut());

            if let Some(Token::Identifier(attr)) = tokens.first().cloned() {
                assert!(tokens.drop_first_mut());
                attrs.push(attr);
            } else {
                return Err((
                    original_tokens,
                    ParsingError::from("not an identifier after dot"),
                ));
            }
        }

        Ok((tokens, AstNode::AttributeLookup(Box::new(node), attrs)))
    } else {
        Ok((tokens, node))
    }
}

fn statement(tokens: List<Token>) -> Result<(List<Token>, AstNode), (List<Token>, ParsingError)> {
    let original_tokens = tokens.clone();

    {
        let var_declaration = |mut tokens: List<Token>| {
            let original_tokens = tokens.clone();
            if tokens.first().cloned() == Some(Token::Var) {
                tokens.drop_first_mut();

                if let Some(Token::Identifier(lhs)) = tokens.first().cloned() {
                    tokens.drop_first_mut();

                    if let Some(Token::Equals) = tokens.first() {
                        tokens.drop_first_mut();

                        match expression(tokens) {
                            Ok((mut tokens, rhs)) => {
                                return Ok((tokens, AstNode::VarDeclaration(lhs, Box::new(rhs))));
                            }

                            Err(_) => {}
                        }
                    }
                }
            }

            return Err((original_tokens, ParsingError::from("Not a var statement.")));
        };

        let assignment = |tokens: List<Token>| {
            let original_tokens = tokens.clone();

            expression(tokens).and_then(|(mut tokens, lhs)| {
                if tokens.first().cloned() == Some(Token::Equals) {
                    tokens.drop_first_mut();
                    expression(tokens)
                        .map(|(tokens, rhs)| {
                            (tokens, AstNode::Assignment(Box::new(lhs), Box::new(rhs)))
                        })
                        .map_err(|_| (original_tokens, ParsingError::from("Not an assignment².")))
                } else {
                    Err((original_tokens, ParsingError::from("Not an assignment.")))
                }
            })
        };

        var_declaration(tokens)
            .or_else(|(tokens, _)| assignment(tokens))
            .or_else(|(tokens, _)| function_call(tokens))
            .and_then(|(mut tokens, value)| {
                if tokens.first().cloned() != Some(Token::Semicolon) {
                    return Err((original_tokens, ParsingError::from("Missing semicolon.")));
                }

                assert!(tokens.drop_first_mut());
                Ok((tokens, value))
            })
    }
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

            Err((new_tokens, e)) => {
                if !new_tokens.is_empty() {
                    return Err(e);
                }

                break;
            }
        }
    }

    Ok(result)
}
