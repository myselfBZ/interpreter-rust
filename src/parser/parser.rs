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
 
    fn parse_ident(&mut self) -> ast::Expression {
        return ast::Expression::Ident(self.cur_tok.to_string())
    }

    fn parse_expression_stmnt(&mut self) -> Option<ast::Statement> {
        match self.cur_tok {
           token::Token::Let => {
               return self.parse_let() 
           },
           _ => return None 
        }
    }

    fn parse_let(&mut self) -> Option<ast::Statement>{
        let let_tok = self.cur_tok.clone();
        match &self.peek_tok{
            token::Token::Ident(_) => self.next_token(),
            _ => return None,
        }
        let name = self.parse_ident().to_string();
        while self.cur_tok != token::Token::Semicolon{
            self.next_token();
        }
        return Some(ast::Statement::Let { token: let_tok, ident: ast::Expression::Ident(name), exprs: ast::Expression::NoExprsn })
    }

    fn next_token(&mut self) {
        self.cur_tok = self.peek_tok.clone();
        self.peek_tok = self.lexer.next_token()
    }

    pub fn parse_program(&mut self) -> Vec<ast::Statement>{
        let mut statements = vec![];
        while  self.cur_tok != token::Token::Eof{
            let stmnt = self.parse_expression_stmnt();
            match stmnt {
                Some(n) =>  statements.push(n),
                None => return statements
            }
            self.next_token();
        } 

        return statements
    }

}


#[cfg(test)]
mod tests {
    use crate::ast;
    use crate::lexer::Lexer; 
    use crate::parser::Parser;
    #[test]
    fn test_parser(){
        let src = "let x = 2;".to_string();
        let lex = Lexer::new(src);
        let mut parser  = Parser::new(Box::new(lex));
        let statements = parser.parse_program();
        if statements.len() != 1{
            panic!("expected only one statement got {}", statements.len())
        }
        let node = ast::ast::Statement::Let { 
            token: crate::token::Token::Let, 
            ident: ast::ast::Expression::Ident("x".to_string()),
            exprs: ast::ast::Expression::NoExprsn
        };
        assert_eq!(statements[0], node)
    }
}



