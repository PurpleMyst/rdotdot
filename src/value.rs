use super::{ast::AstNode, reffers::rc::Strong};
use std::fmt;

pub struct BuiltinFunctionData {
    pub name: &'static str,
    pub func: Box<Fn(Vec<Strong<Value>>) -> Strong<Value>>,
}

impl fmt::Debug for BuiltinFunctionData {
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
    BuiltinFunction(BuiltinFunctionData),
    Unit,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::List(l) => {
                write!(f, "[")?;
                l.iter()
                    .enumerate()
                    .map(|(i, element)| {
                        if i > 0 {
                            write!(f, " ")?;
                        }
                        element.get().fmt_list_element(f)
                    })
                    .collect::<fmt::Result>()?;
                write!(f, "]")
            }

            Value::StatementBlock(..) => unimplemented!(),
            Value::ExpressionBlock(..) => unimplemented!(),
            Value::BuiltinFunction(..) => unimplemented!(),

            Value::Unit => write!(f, "()"),
        }
    }
}

impl Value {
    fn fmt_list_element(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{:?}", s),
            _ => write!(f, "{}", self),
        }
    }

    pub fn builtin_function(
        name: &'static str,
        func: impl Fn(Vec<Strong<Value>>) -> Strong<Value> + 'static,
    ) -> Self {
        Value::BuiltinFunction(BuiltinFunctionData {
            name: name,
            func: Box::new(func),
        })
    }
}
