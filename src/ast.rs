#[derive(Debug)]
pub enum AstNode {
    StringLiteral(String),
    AttributeLookup(Box<AstNode>, Vec<String>),
    VariableLookup(String),
    IntegerLiteral(i64),
    FunctionCall {
        func: Box<AstNode>,
        args: Vec<AstNode>,
    },
    StatementBlock(Vec<AstNode>),
    ExpressionBlock(Box<AstNode>),
    VarDeclaration(String, Box<AstNode>),
    Assignment(Box<AstNode>, Box<AstNode>),
    List(Vec<AstNode>),
}
