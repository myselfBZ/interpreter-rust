use core::fmt;

#[derive(Hash,Eq,Debug, PartialEq, Clone)]
pub enum Token {
    Ident(String),
    Semicolon,
    Bang,
    Let,
    Int(String),
    Assing,
    Plus,
    Minus,
    False,
    True,
    If,
    Slash,
    Asterisk,
    Else,
    Func,
    Illgl(String),
    Return,
    Eof,
    Eq,
    NotEq,
    Gt,
    Lt,
    Rparen,
    Lparen,
    Lbrace,
    Rbrace,
    Comma
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self{
            Token::Return => write!(f, "return"),
            Token::Gt=> write!(f, ">"),
            Token::Comma=> write!(f, ","),
            Token::Lparen=> write!(f, "("),
            Token::Lbrace=> write!(f, "{{"),
            Token::Rbrace=> write!(f, "}}"),
            Token::Rparen=> write!(f, ")"),
            Token::Lt=> write!(f, "<"),
            Token::Eq => write!(f, "=="),
            Token::NotEq => write!(f, "!="),
            Token::Bang => write!(f, "!"),
            Token::Eof => write!(f, "the end"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Let => write!(f, "Let"),
            Token::Ident(x) => write!(f, "{}", x),
            Token::Semicolon => write!(f, ";"),
            Token::Int(x) => write!(f, "Int {}", x),
            Token::Assing => write!(f, "="),
            Token::True => write!(f, "TRUE"),
            Token::If =>  write!(f, "If"),
            Token::False => write!(f, "FALSE"),
            Token::Else =>  write!(f, "Else"),
            Token::Func =>  write!(f, "Func"),
            Token::Illgl(x) => write!(f, "ILLEGAL {}", x),
        }
    }
}
