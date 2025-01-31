mod lexer;
mod token;
mod ast;
mod parser;
fn main() {
    let src = String::from("true; false;");
    let lexer = lexer::Lexer::new(src);
    let mut parser = parser::Parser::new(Box::new(lexer));
    let statmnts = parser.parse_program();
    println!("node: {:?}", statmnts)
}
