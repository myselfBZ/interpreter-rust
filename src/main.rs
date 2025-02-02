use eval::eval::eval_program;

mod eval;
mod lexer;
mod objects;
mod token;
mod ast;
mod parser;
fn main() {
    let src = String::from("
        ((1 + 2) * 2) / 6;
        ");
    let lexer = lexer::Lexer::new(src);
    let mut parser = parser::Parser::new(Box::new(lexer));
    let statmnts = parser.parse_program();
    let result = eval_program(statmnts);
    println!("{}", result)
}
