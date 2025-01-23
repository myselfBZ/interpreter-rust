use crate::token;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Ident(String),
    SomeExprs
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self{
            Expression::Ident(s) => write!(f, "identifier {}", s),
            Expression::SomeExprs => write!(f, "we got something really illegal")
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum Statement {
   Let {
      token:token::Token, 
      ident:String,
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

pub fn check_let(t:token::Token) -> Result<Statement, String>{
    match t{
        token::Token::Let => return Ok(Statement::Let { token: t, ident: "something".to_string(), exprs: Expression::SomeExprs }),
        _ => return Err(String::from("oh nooo"))
    }
}


