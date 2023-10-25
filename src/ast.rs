use crate::lexer::{TokenType, Token};

#[derive(Debug, Clone)]
pub enum Expression{
    Literal(Literal),
    BlockExpression(BlockExpression),
    IfExpression(IfExpression),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    CallExpression(CallExpression),
    IndexExpression(IndexExpression),

}

pub enum Value {
    Identifier(Identifier),
    String(StringLiteral),
    Number(NumberValue),
    Function(FunctionValue),
    Array(ArrayValue),
    Class(ClassValue)
}




pub struct BlockExpression{
    pub exprs: Vec<Expression>,
    pub t: Token
}

pub struct IfExpression{
    pub condition: Box<Expression>,
    pub consequence: Box<BlockExpression>,
    pub t: Token
}

pub struct PrefixExpression {
    pub op: Token,
    pub r: Box<Expression>
}

pub struct InfixExpression {
    pub l: Box<Expression>,
    pub ops: Token,
    pub r: Box<Expression>
}

pub struct CallExpression {
    pub f: Box<Expression>,
    pub args: Vec<Expression>,
    pub t: Token
}

pub struct IndexExpression {
    pub index: Box<Expression>
}


pub enum Statement {
    Block(Vec<Expression>),
    Expression {
        expression: Box<dyn Expression>
    },
    Var {
        kind: Token,
        name: Token,
        init: Box<dyn Expression>
    },
    None
}