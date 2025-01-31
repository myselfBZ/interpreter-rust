use core::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Ident(String),
    Semicolon,
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
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self{
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
