use super::{
    ast::AstNode, chain_map::ChainMap, reffers::rc::Strong, value::{BuiltinFunctionData, Value},
};

pub struct Scope {
    variables: ChainMap<String, Strong<Value>>,
}

impl Scope {
    pub fn new() -> Self {
        let mut ctx = Self::prelude();
        ctx.enter_scope();
        ctx
    }

    pub fn prelude() -> Self {
        let mut ctx = Self {
            variables: ChainMap::new(),
        };
        ctx.set(
            AstNode::VariableLookup {
                ident: String::from("print"),
            },
            Strong::new(Value::builtin_function("print", |args| {
                args.into_iter().for_each(|arg| print!("{}", *arg.get()));
                println!();
                Strong::new(Value::Unit)
            })),
        );
        ctx
    }

    fn get(&self, node: AstNode) -> Strong<Value> {
        match node {
            AstNode::VariableLookup { ident } => {
                if let Some(value) = self.variables.get(&ident) {
                    value.clone()
                } else {
                    panic!("Undefined variable {:?}", ident);
                }
            }
            _ => {
                unimplemented!("Scope::get(self, {:?})", node);
            }
        }
    }

    fn set(&mut self, place: AstNode, value: Strong<Value>) -> bool {
        match place {
            AstNode::VariableLookup { ident } => self.variables.set(ident, value),
            _ => unimplemented!("Scope::set(self, {:?}, {:?})", place, value),
        }
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
                let args = args
                    .into_iter()
                    .map(|arg| self.eval(arg))
                    .collect::<Vec<_>>();

                self.enter_scope();

                match &*func.get() {
                    Value::BuiltinFunction(BuiltinFunctionData { func, .. }) => {
                        func(args);
                    }

                    _ => unreachable!(),
                }

                self.exit_scope();

                Strong::new(Value::Unit)
            }

            AstNode::VariableCreation { ident, value } => {
                let value = self.eval(*value);
                assert!(!self.set(AstNode::VariableLookup { ident }, value));
                Strong::new(Value::Unit)
            }

            AstNode::Assignment { expr, value } => {
                let value = self.eval(*value);
                self.set(*expr, value);
                Strong::new(Value::Unit)
            }

            AstNode::Comment(_) => unreachable!("Tried to run comment"),
        }
    }
}
