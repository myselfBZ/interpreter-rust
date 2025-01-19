

pub struct Lexer {
    ch:u8
}


impl Lexer{
    pub fn new() -> Self{
        let ch = 0;
        Lexer{ch}
    }
    pub fn start(&self){
        println!("first char: {}", char::from(self.ch))
    }
}
