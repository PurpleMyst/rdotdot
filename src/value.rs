use super::{ast::AstNode, reffers::rc::Strong};
use std::fmt;

pub struct BuiltinFunctionInner {
    pub name: &'static str,
    pub func: Box<Fn(Vec<Strong<Value>>) -> Strong<Value>>,
}

impl fmt::Debug for BuiltinFunctionInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.name)
    }
}

#[derive(Debug)]
pub enum Value {
    Integer(i64),
    String(String),
    List(Vec<Strong<Value>>),
    StatementBlock(Vec<AstNode>),
    ExpressionBlock(Box<AstNode>),
    BuiltinFunction(BuiltinFunctionInner),
    Unit,
}
