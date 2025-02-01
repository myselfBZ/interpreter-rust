mod lexer;
mod token;
mod ast;
mod parser;
fn main() {
    let src = String::from("
        if b > a {
            let x = 12 + 2;
            return 21;
        } else {
            return 1;
        }
        ");
    let lexer = lexer::Lexer::new(src);
    let mut parser = parser::Parser::new(Box::new(lexer));
    let statmnts = parser.parse_program();
    println!("node: {:?}", statmnts);
    println!("number of statements: {}", statmnts.len())
}
