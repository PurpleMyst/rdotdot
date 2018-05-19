use super::{
    ast::AstNode, chain_map::ChainMap, reffers::rc::Strong, value::{BuiltinFunctionInner, Value},
};

pub struct Environment {
    variables: ChainMap<String, Strong<Value>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: ChainMap::new(),
        }
    }

    pub fn prelude() -> Self {
        let mut env = Self::new();
        env.set(
            String::from("print"),
            Strong::new(Value::BuiltinFunction(BuiltinFunctionInner {
                name: "print",
                func: Box::new(|args| {
                    args.into_iter().for_each(|arg| print!("{}", *arg.get()));
                    println!();
                    Strong::new(Value::Unit)
                }),
            })),
        );
        env
    }

    fn get(&self, node: AstNode) -> Strong<Value> {
        match node {
            AstNode::VariableLookup { ident } => self.variables
                .get(ident)
                .expect("Undefined variable")
                .clone(),

            _ => unreachable!("tried to get the variable stored at {:?}", node),
        }
    }

    fn set(&mut self, ident: String, value: Strong<Value>) -> bool {
        self.variables.set(ident, value)
    }

    fn enter_scope(&mut self) {
        self.variables.push_map();
    }

    fn exit_scope(&mut self) {
        self.variables.pop_map();
    }

    pub fn eval(&mut self, node: AstNode) -> Strong<Value> {
        match node {
            AstNode::String(s) => Strong::new(Value::String(s)),

            AstNode::Integer(n) => Strong::new(Value::Integer(n)),

            AstNode::StatementBlock(b) => Strong::new(Value::StatementBlock(b)),
            AstNode::ExpressionBlock(b) => Strong::new(Value::ExpressionBlock(b)),

            AstNode::List(l) => {
                Strong::new(Value::List(l.into_iter().map(|i| self.eval(i)).collect()))
            }

            AstNode::VariableLookup { .. } => self.get(node),

            AstNode::AttributeLookup { .. } => self.get(node),

            AstNode::MethodLookup { .. } => self.get(node),

            AstNode::FunctionCall { func, args } => {
                let func = self.eval(*func);
                let args = args.into_iter()
                    .map(|arg| self.eval(arg))
                    .collect::<Vec<_>>();

                self.enter_scope();

                match &*func.get() {
                    Value::BuiltinFunction(BuiltinFunctionInner { func, .. }) => {
                        func(args);
                    }

                    _ => unreachable!(),
                }

                self.exit_scope();

                Strong::new(Value::Unit)
            }

            AstNode::VariableCreation { ident, value } => {
                let value = self.eval(*value);
                assert!(!self.set(ident, value));
                Strong::new(Value::Unit)
            }

            AstNode::Assignment { expr: _, value: _ } => unimplemented!(),

            AstNode::Comment(_) => unreachable!(),
        }
    }
}
