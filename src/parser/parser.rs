use crate::{ast::ast::{self, Expression}, lexer::lexer, token::token};

pub struct Parser{
    cur_tok: token::Token,
    peek_tok: token::Token,
    lexer : Box<lexer::Lexer>,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call
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
        let exprs = self.parse_expression(Precedence::Lowest);
        if self.peek_tok == token::Token::Semicolon{
            self.next_token();
        }
        return Some(ast::Statement::ExprsStatement { token: tok,  exprs })
    }

    fn parse_grouped_expression(&mut self) -> ast::Expression {
        self.next_token();
        let node = self.parse_expression(Precedence::Lowest);
        if self.peek_tok == token::Token::Rparen{
            self.next_token();
        }
        return node
    }

    fn parse_expression(&mut self, prec:Precedence) -> ast::Expression{
        let mut left = match &self.cur_tok{
            token::Token::Int(_) => self.parse_int(),
            token::Token::True => self.parse_bool(),
            token::Token::False => self.parse_bool(),
            token::Token::Minus => self.parse_prefix_ops(),
            token::Token::Bang => self.parse_prefix_ops(),
            token::Token::Lparen => self.parse_grouped_expression(),
            token::Token::Ident(_) => self.parse_ident(),
            _ => ast::Expression::NoExprsn
        };

        while self.cur_tok != token::Token::Semicolon && self.token_to_precedence(self.peek_tok.clone()) > prec{
            match &self.peek_tok {
                token::Token::Plus|
                    token::Token::Minus|
                    token::Token::Asterisk|
                    token::Token::Slash|
                    token::Token::Eq|
                    token::Token::Gt|
                    token::Token::Lt|
                    token::Token::NotEq => {
                        self.next_token();
                        left = self.parse_infix(left)
                    }

                _ => return Expression::NoExprsn
            }
        }

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


    fn token_to_precedence(&self,tok:token::Token) -> Precedence{
        match tok {
            token::Token::Slash => return Precedence::Product,
            token::Token::Gt => return Precedence::LessGreater,
            token::Token::Lt => return Precedence::LessGreater,
            token::Token::Asterisk => return Precedence::Product,
            token::Token::Eq => return Precedence::Equals,
            token::Token::NotEq => return Precedence::Product,
            token::Token::Plus => return Precedence::Sum,
            token::Token::Minus => return Precedence::Sum,
            _ => Precedence::Lowest
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
            self.next_token();
        } 

        return statements
    }

    fn parse_prefix_ops(&mut self) -> ast::Expression{
        let tok = self.cur_tok.clone(); 
        self.next_token();
        let right = self.parse_expression(Precedence::Lowest);
        return ast::Expression::PrefixExprsn { token: tok, exprsn: Box::new(right) }
    }

    fn parse_infix(&mut self, left:ast::Expression) -> ast::Expression {
        let opr = self.cur_tok.clone();
        self.next_token();
        let right  = self.parse_expression(self.token_to_precedence(opr.clone()));
        return ast::Expression::InfixExprsn { left: Box::new(left), right: Box::new(right), oprt: opr.to_string() }
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

    #[test]
    fn test_prefix_ops(){
        let src = "!true;".to_string();
        let lex = lexer::Lexer::new(src);
        let mut p = parser::Parser::new(Box::new(lex));
        let stmnts = p.parse_program();
        if stmnts.len() != 1{
            panic!("expected 1 got {}", stmnts.len())
        }
        let expected = [
            ast::ast::Statement::ExprsStatement { token: token::Token::Bang, exprs: 
                ast::ast::Expression::PrefixExprsn { 
                    token: token::Token::Bang,
                    exprsn: Box::new(ast::ast::Expression::Boolean { 
                        token: token::Token::True, 
                        value: true 
                    })
                }
        }
        ];
        assert_eq!(stmnts[0], expected[0])
    }
    #[test]
    fn test_infix(){
        let src = "1+1; 1+2*3;".to_string();
        let lex = lexer::Lexer::new(src);
        let mut p = parser::Parser::new(Box::new(lex));
        let stmnts = p.parse_program();
        if stmnts.len() != 2{
            panic!("expected 2 got {}", stmnts.len())
        }
        let expected = [
            ast::ast::Statement::ExprsStatement { token: token::Token::Int("1".to_string()), 
            exprs: ast::ast::Expression::InfixExprsn { 
                left: Box::new(ast::ast::Expression::Int(1)), 
                right: Box::new(ast::ast::Expression::Int(1)), 
                oprt: "+".to_string() 
            }
        },
        ast::ast::Statement::ExprsStatement { token: token::Token::Int("1".to_string()), 
            exprs: ast::ast::Expression::InfixExprsn { 
                left: Box::new(ast::ast::Expression::Int(1)), 
                right: Box::new(ast::ast::Expression::InfixExprsn { left: Box::new(ast::ast::Expression::Int(2)), right: Box::new(ast::ast::Expression::Int(3)), oprt: "*".to_string() }), 
                oprt: "+".to_string() 
            }
        },
        ];
        for (i, v) in stmnts.iter().enumerate(){
           assert_eq!(*v, expected[i]) 
        }
    }

    #[test]
    fn test_grouped(){
        let src = "(1 + 1) * 2".to_string();
        let lex = lexer::Lexer::new(src);
        let mut p = parser::Parser::new(Box::new(lex));
        let stmnts = p.parse_program();


        if stmnts.len() != 1{
            panic!("expected 2 got {}", stmnts.len())
        }


        let expected = ast::ast::Statement::ExprsStatement { 
            token: token::Token::Lparen, 
            exprs: ast::ast::Expression::InfixExprsn { 
                right: Box::new(ast::ast::Expression::Int(2)), 
                left: Box::new(
                    ast::ast::Expression::InfixExprsn { 
                        left: Box::new(ast::ast::Expression::Int(1)), 
                        right: Box::new(ast::ast::Expression::Int(1)), 
                        oprt: "+".to_string() 
                }), 
                oprt: "*".to_string() 
            }
        };
        assert_eq!(stmnts[0], expected)
    }
}

