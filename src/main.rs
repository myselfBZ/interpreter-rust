mod lexer;
mod token;
fn main() {
    let src = String::from("let x = 12; , true false");
    let mut l = lexer::Lexer::new(src);
    l.preset();
}
