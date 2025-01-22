mod lexer;
mod token;
mod ast;
fn main() {
    let src = String::from("let x = 12; true false;;;;");
    let mut l = lexer::Lexer::new(src);
    l.preset();
}
