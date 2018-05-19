// TODO: Nice error messages and tests for most of the parsers.
// TODO: Define a `parenthesised!` macro.
// XXX: I notice in many examples there's the form `{ block }::run;` which is not described in the
// informal spec. I've taken care to implement it as a statement, but it's a bit weird.

use nom::{self, multispace0, multispace1};

use ast::AstNode;

named!(
    pub expression<AstNode>,
    alt!(
        // NB: Order is important.
        attribute_lookup |
        method_lookup |
        variable_lookup |
        string |
        integer |
        list |
        block |
        delimited!(tuple!(char!('('), multispace0), function_call, tuple!(multispace0, char!(')')))
    )
);

named!(
    pub statement<AstNode>,
    alt!(
        comment |
        variable_creation |
        assignment |
        map!(tuple!(method_lookup, multispace0, char!(';')), |t| t.0) |
        map!(tuple!(function_call, multispace0, char!(';')), |t| t.0)
    )
);

named!(
    comment<AstNode>,
    do_parse!(
        char!('#') >> // hash
        contents: many0!(none_of!("\n")) >> // contents
        char!('\n') >> // newline
        (AstNode::Comment(contents.into_iter().collect()))
    )
);

named!(
    function_call<AstNode>,
    do_parse!(
        func: expression >> // func
        args: many0!(tuple!(multispace0, expression)) >> // args
        (AstNode::FunctionCall {
            func: Box::new(func),
            args: args.into_iter().map(|(_, e)| e).collect(),
        })
    )
);

named!(
    variable_creation<AstNode>,
    do_parse!(
        tag!("var") >> // var
        multispace1 >> lhs: identifier >> // lhs
        multispace0 >> char!('=') >> // equals
        multispace0 >> rhs: expression >> // rhs
        multispace0 >> char!(';') >> // semicolon
        (AstNode::VariableCreation {
            ident: lhs,
            value: Box::new(rhs),
        })
    )
);

named!(
    attribute_lookup<AstNode>,
    do_parse!(
        expr: alt!(variable_lookup | delimited!(tuple!(char!('('), multispace0), expression, tuple!(multispace0, char!(')')))) >> // expr
        multispace0 >> char!('.') >> // dot
        multispace0 >> attr: identifier >> // attr
        (AstNode::AttributeLookup {
            expr: Box::new(expr),
            attr,
        })
    )
);

named!(
    method_lookup<AstNode>,
    do_parse!(
        expr: alt!(variable_lookup | block | delimited!(tuple!(char!('('), multispace0), expression, tuple!(multispace0, char!(')')))) >> // expr
        multispace0 >> tag!("::") >> // dot
        multispace0 >> meth: identifier >> // attr
        (AstNode::MethodLookup {
            expr: Box::new(expr),
            meth,
        })
    )
);

named!(
    assignment<AstNode>,
    do_parse!(
        lhs: expression >> // lhs
        multispace0 >> char!('=') >> // equals
        multispace0 >> rhs: expression >> // rhs
        multispace0 >> char!(';') >> // semicolon
        (AstNode::Assignment {
            expr: Box::new(lhs),
            value: Box::new(rhs),
        })
    )
);

named!(
    identifier<String>,
    do_parse!(
        first: one_of!("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_") >> // first
        rest: many0!(one_of!("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_1234567890")) >> // rest
        (::std::iter::once(first).chain(rest.into_iter()).collect())
    )
);

named!(
    variable_lookup<AstNode>,
    map!(identifier, |ident| AstNode::VariableLookup { ident })
);

named!(
    pub program< Vec<AstNode> >,
    many0!(map!(complete!(tuple!(multispace0, statement)), |t| t.1))
);

// TODO: Support escapes.
named!(
    string<AstNode>,
    map!(
        delimited!(char!('"'), many0!(none_of!("\"")), char!('"')),
        |s| AstNode::String(s.into_iter().collect())
    )
);

named!(
    list<AstNode>,
    map!(
        delimited!(
            char!('['),
            many0!(tuple!(multispace0, expression)),
            tuple!(multispace0, char!(']'))
        ),
        |es| AstNode::List(es.into_iter().map(|e| e.1).collect())
    )
);

named!(
    block<AstNode>,
    delimited!(
        char!('{'),
        alt!(
            many1!(tuple!(multispace0, statement)) => {|es: Vec<(&[u8], AstNode)>| AstNode::StatementBlock(es.into_iter().map(|e| e.1).collect())} |
            tuple!(multispace0, expression) => {|(_, e)| AstNode::ExpressionBlock(Box::new(e)) } |
            multispace0 => { |_| AstNode::StatementBlock(Vec::new()) }
        ),
        tuple!(multispace0, char!('}'))
    )
);

// XXX: Can we use `parse_to!` here?
named!(
    integer<AstNode>,
    map_opt!(nom::digit1, |n: &[u8]| Some(AstNode::Integer(
        String::from_utf8(n.to_vec()).ok()?.parse().ok()?
    )))
);
