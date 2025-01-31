use crate::{ast::ast::{self}, lexer::lexer, token::token};


pub struct Parser{
    cur_tok: token::Token,
    peek_tok: token::Token,
    lexer : Box<lexer::Lexer>,
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

    fn parse_expression_statement(&mut self) -> Option<ast::Statement>{
        let tok = self.cur_tok.clone();
        let exprs = self.parse_expression();
        if self.peek_tok == token::Token::Semicolon{
            self.next_token();
        }
        return Some(ast::Statement::ExprsStatement { token: tok,  exprs })
    }

    fn parse_expression(&mut self) -> ast::Expression{
        let left = match &self.cur_tok{
            token::Token::Int(_) => self.parse_int(),
            token::Token::True => self.parse_bool(),
            token::Token::False => self.parse_bool(),
            _ => ast::Expression::NoExprsn
        };
        left
    } 

    fn parse_bool(&self) -> ast::Expression {
        return ast::Expression::Boolean{ token:self.cur_tok.clone(), value: self.cur_tok == token::Token::True}
    }

    fn parse_int(&mut self) -> ast::Expression {
        let literal = match &self.cur_tok{
            token::Token::Int(s) => s,
            _ => return ast::Expression::NoExprsn 
        };
        let int: i32 = match literal.parse(){
            Ok(n) => n,
            Err(_s) => return ast::Expression::NoExprsn
        };
        return ast::Expression::Int(int)    
    }


    fn parse_statemnt(&mut self) -> Option<ast::Statement> {
        match self.cur_tok {
           token::Token::Let => {
               return self.parse_let() 
           },
           token::Token::Return => {
                return self.parse_return()
           },
           _ => return self.parse_expression_statement() 
        }
    }

    fn parse_return(&mut self) -> Option<ast::Statement> {
       let return_tok = self.cur_tok.clone();     
        while self.cur_tok != token::Token::Semicolon{
            self.next_token();
        }
        return Some(ast::Statement::Return{ token: return_tok, exprs:ast::Expression::NoExprsn })
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
            let stmnt = self.parse_statemnt();
            match stmnt {
                Some(n) =>  statements.push(n),
                None => return statements
            }
            println!("cur token {} peek  token is {} ", self.cur_tok, self.peek_tok);
            self.next_token();
        } 

        return statements
    }

}


#[cfg(test)]
mod tests {
    use crate::token::token;
    use crate::{ast, lexer, parser};
    use crate::lexer::Lexer; 
    use crate::parser::Parser;
    #[test]
    fn test_let(){
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
    #[test]
    fn test_return(){
        let src = "return 12;".to_string();
        let lex = Lexer::new(src);
        let mut parser  = Parser::new(Box::new(lex));
        let statements = parser.parse_program();
        if statements.len() != 1{
            panic!("expected only one statement got {}", statements.len())
        }
        let node = ast::ast::Statement::Return { 
            token: crate::token::Token::Return, 
            exprs: ast::ast::Expression::NoExprsn
        };
        assert_eq!(statements[0], node)
    }

    #[test]
    fn test_int() {
        let src = "12;".to_string();
        let lex = lexer::Lexer::new(src);
        let mut p = parser::Parser::new(Box::new(lex));
        let stmnts = p.parse_program();
        let expected = ast::ast::Statement::ExprsStatement { 
            token: token::Token::Int("12".to_string()), 
            exprs: ast::ast::Expression::Int(12) 
        };
        if stmnts.len() != 1{
            panic!("expected 1 statement got {}", stmnts.len())
        }
        assert_eq!(stmnts[0], expected)
    }

    #[test]
    fn test_bool() {
        let src = "false; true;".to_string();
        let lex = lexer::Lexer::new(src);
        let mut p = parser::Parser::new(Box::new(lex));
        let stmnts = p.parse_program();
        let expected = [ 
            ast::ast::Statement::ExprsStatement { 
                token: token::Token::False, 
                exprs: ast::ast::Expression::Boolean { token: token::Token::False, value: false } 
            },
            ast::ast::Statement::ExprsStatement { 
                token: token::Token::True, 
                exprs: ast::ast::Expression::Boolean { token: token::Token::True, value: true } 
            },
        ];
        if stmnts.len() != 2{
            panic!("expected 1 statement got {}", stmnts.len())
        }
        assert_eq!(stmnts, expected)
    }


}

