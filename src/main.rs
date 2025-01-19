mod lexer;

fn main() {
    let l = lexer::Lexer::new();
    l.start();
}
