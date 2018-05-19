#[derive(Debug, PartialEq, Eq)]
pub enum AstNode {
    String(String),

    Integer(i64),

    List(Vec<AstNode>),

    Block(Vec<AstNode>),

    VariableLookup {
        ident: String,
    },

    AttributeLookup {
        expr: Box<AstNode>,
        attr: String,
    },

    MethodLookup {
        expr: Box<AstNode>,
        meth: String,
    },

    FunctionCall {
        func: Box<AstNode>,
        args: Vec<AstNode>,
    },

    VariableCreation {
        ident: String,
        value: Box<AstNode>,
    },

    Assignment {
        expr: Box<AstNode>,
        value: Box<AstNode>,
    },
}
