use crate::{ast::ast, lexer::lexer, token::token};


pub struct Parser{
    cur_tok: token::Token,
    peek_tok: token::Token,
    lexer : Box<lexer::Lexer>
}

impl Parser {
    pub fn new(mut lexer:Box<lexer::Lexer>) -> Self{
        let cur_tok = lexer.next_token();
        let peek_tok = lexer.next_token();
        let p = Parser{lexer:Box::new(*lexer), cur_tok, peek_tok};
        p
    }
 
    pub fn hello(&self) {
       println!("Hello world"); 
    } 

    fn parse_ident(&mut self) -> ast::Expression {
        return ast::Expression::Ident(self.cur_tok.to_string())
    }

    fn parse_let(&mut self) -> Option<ast::Statement>{
        let let_tok = self.cur_tok.clone();
        match &self.peek_tok{
            token::Token::Ident(_) => self.next_token(),
            _ => return None,
        }
        let name = self.parse_ident().to_string();
        while self.cur_tok != token::Token::Semicolon || self.cur_tok != token::Token::Eof{
            self.next_token();
        }
        return Some(ast::Statement::Let { token: let_tok, ident: name, exprs: ast::Expression::NoExprsn })
    }

    fn next_token(&mut self) {
        self.cur_tok = self.peek_tok.clone();
        self.peek_tok = self.lexer.next_token()
    }


}
