use crate::token;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Ident(String),
    NoExprsn,
    Int(i32),
    Boolean {
        token: token::Token,
        value:bool
    },
    PrefixExprsn {
        token: token::Token,
        exprsn: Box<Expression>
    },
    InfixExprsn {
        left: Box<Expression>,
        right: Box<Expression>,
        oprt: String
    },
    IfExprsn{
        condt: Box<Expression>,
        conseq: Vec<Statement>,
        alter: Vec<Statement>
    },
    FnExprsn{
        params: Vec<Expression>,
        body: Vec<Statement>
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Expression::Ident(s)=> write!(f, "{}", s),
            Expression::IfExprsn{ condt, conseq, alter }=> write!(f, "{condt}\n, {:?}\n {:?}\n", conseq, alter),
            Expression::FnExprsn{ params, body }=> write!(f, "FUNCTION params: {:?} Body: {:?}", params, body),
            Expression::Boolean{token, value} =>  write!(f, " here is the statement: {} {}", token, value),
            Expression::PrefixExprsn{token, exprsn} =>  write!(f, "PrefixExprsn: {token} {exprsn}"),
            Expression::InfixExprsn{left, oprt,right} =>  write!(f, "InfixExprsn: {left} {oprt} {right} "),
            Expression::Int(s)=> write!(f, "{}", s),
            Expression::NoExprsn => write!(f, "we are skippin' the expression for now")
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum Statement {
    Let {
        token:token::Token, 
        ident:Expression,
        exprs:Expression
    },

    Return {
        token:token::Token,
        exprs:Expression
    },
    ExprsStatement{
        token:token::Token,
        exprs:Expression
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Statement::Let{token, ident, exprs} =>  write!(f, " here is the statement: {} {} {}", token, ident, exprs),
            Statement::ExprsStatement{token, exprs} =>  write!(f, "{} {}", token, exprs),
            Statement::Return{token, exprs} =>  write!(f, " here is the statement: {} {}", token, exprs),
        }
    }
}
