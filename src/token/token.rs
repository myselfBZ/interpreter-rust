use core::fmt;

#[derive(Debug)]
pub enum Token {
    Ident(String),
    Semicolon,
    Let,
    Int(String),
    Assing,
    False,
    True,
    If,
    Else,
    Func,
    Illgl(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self{
            Token::Let => write!(f, "Let"),
            Token::Ident(x) => write!(f, "Ident {}", x),
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
