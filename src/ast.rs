#[derive(Debug)]
pub enum AstNode {
    StringLiteral(String),
    VariableLookup(String),
    IntegerLiteral(i64),
    FunctionCall {
        func: Box<AstNode>,
        args: Vec<AstNode>,
    },
}
