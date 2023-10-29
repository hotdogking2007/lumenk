use crate::lexer::{TokenType, Token};
#[derive(Debug, Clone)]
pub enum Expr{
    Literal(Value),
    Assign,
    IfExpression {
        condition: Box<Expr>,
        consequence: Box<Expr>
    },
    PrefixExpression(PreOp, Box<Expr>),
    InfixExpression{l: Box<Expr>, op: BinOp, r: Box<Expr>},
    CallExpression{
        name: String,
        args: Vec<Expr>
    },
    FunctionExpression {
        name: String,
        args: Vec<Expr>,
        code: Box<Expr>
    },
    IndexExpression(Box<Expr>, Box<Expr>),
}
#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Minus,
    Multi,
    Div,
    Mod,
}
#[derive(Debug, Clone)]
pub enum PreOp {
    Not,
    Minus,
    MemoryAdress,
    Defer,
}
#[derive(Debug, Clone)]
pub enum Value {
    Identifier(String),
    String(String),
    Number(String),
    Array(Vec<Expr>),
    None
}

#[derive(Debug)]
pub enum Stmt {
    Block(Vec<Expr>),
    Var {
        kind: Token,
        name: Token,
        init: Box<Expr>
    },
    Class{
        name: String,
        property: Vec<Expr>,
        method: Vec<Expr>,
    },
    None
}
