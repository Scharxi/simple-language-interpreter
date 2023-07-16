use crate::tokenizer::token::Token;



#[derive(Debug)]
pub struct Variable {
    pub name: Token,
}

#[derive(Debug)]
pub struct FunCall {
    pub name: Variable,
    pub arg: Box<Expr>,
}

#[derive(Debug)]
pub struct Assignment {
    pub target: Variable,
    pub value: Box<Expr>,
}

#[derive(Debug)]
pub struct BinaryOp {
    pub lhs: Box<Expr>,
    pub operation: Token,
    pub rhs: Box<Expr>,
}

#[derive(Debug)]
pub struct Number {
    pub value: i32,
    pub token: Token,
}

#[derive(Debug)]
pub enum Expr {
    Assignment(Assignment),
    Variable(Variable),
    Number(Number),
    BinaryOp(BinaryOp),
    FunCall(FunCall),
}