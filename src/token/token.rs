use core::fmt;

#[derive(Debug)]
pub enum Token {
    Ident(String),
    Let,
    Int(String),
    Assing,
    False,
    True,
    Illgl(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self{
            Token::Let => write!(f, "Let"),
            Token::Ident(x) => write!(f, "Ident {}", x),
            Token::Int(x) => write!(f, "Int {}", x),
            Token::Assing => write!(f, "="),
            Token::True => write!(f, "TRUE"),
            Token::False => write!(f, "FALSE"),
            Token::Illgl(x) => write!(f, "ILLEGAL {}", x),
        }
    }
}
