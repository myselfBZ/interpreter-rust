mod lexer;
mod token;
mod ast;
mod parser;
fn main() {
    let src = String::from("let x = 12; true false;;;;");
    let l = lexer::Lexer::new(src);
    let p = parser::Parser::new(Box::new(l));
    p.hello();
}
