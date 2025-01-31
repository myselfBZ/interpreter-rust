use crate::token;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Ident(String),
    NoExprsn
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self{
            Expression::Ident(s)=> write!(f, "{}", s),
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
   } 
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self{
            Statement::Let{token, ident, exprs} =>  write!(f, " here is the statement: {} {} {}", token, ident, exprs),
        }
    }
}
