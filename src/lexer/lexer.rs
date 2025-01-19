
use crate::token;

pub struct Lexer{
   input:Vec<u8>,
   ch:u8,
   pos:usize,
   peek:usize,
}


impl Lexer {
    pub fn new(input:String) -> Self {
        let mut l = Lexer{input:input.into_bytes(), ch:0, pos:0, peek:0};
        l.read_char();
        return l
    }

    fn read_char(&mut self){
        if self.peek >= self.input.len(){
            self.ch = b'\0'
        } else{
            self.ch = self.input[self.peek]
        }
        self.pos = self.peek;
        self.peek +=1;
    }


    fn skip_white(&mut self){
        while self.ch.is_ascii_whitespace(){
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> token::Token{
        self.skip_white();
        let tok = match self.ch{
            b'a'..=b'z' => {
                let v = self.read_word();
                match v.as_str() {
                    "let" => token::Token::Let,
                    "true" => token::Token::True,
                    "false" => token::Token::False,
                    _ => token::Token::Ident(v)
                }
            },
            b'=' => {
                token::Token::Assing
            },
            b'0'..=b'9' => {
                let number = self.read_number();
                token::Token::Int(number)
            },
            _ => token::Token::Illgl((self.ch as char).to_string())
        };
        self.read_char();
        return tok
    }

    fn read_number(&mut self) -> String{
        let pos = self.pos;
        while self.ch.is_ascii_digit(){
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[pos..self.pos]).to_string();
    }

    fn read_word(&mut self) -> String{
        let pos = self.pos;
        while self.ch.is_ascii_alphabetic() {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[pos..self.pos]).to_string();
    }

    pub fn preset(&mut self) {
       while self.ch != b'\0'{
           println!("token {} ", self.next_token()) 
       } 
    }
}





