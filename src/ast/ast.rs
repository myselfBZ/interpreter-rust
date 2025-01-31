use crate::token;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Ident(String),
    NoExprsn,
    Int(i32)
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self{
            Expression::Ident(s)=> write!(f, "{}", s),
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
        return match self{
            Statement::Let{token, ident, exprs} =>  write!(f, " here is the statement: {} {} {}", token, ident, exprs),
            Statement::ExprsStatement{token, exprs} =>  write!(f, "{} {}", token, exprs),
            Statement::Return{token, exprs} =>  write!(f, " here is the statement: {} {}", token, exprs),
        }
    }
}
