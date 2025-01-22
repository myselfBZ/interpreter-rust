use crate::token;

#[derive(Debug, PartialEq)]
pub enum Statement {
   Let {
      token:token::Token, 
      ident:String,
   } 
}


