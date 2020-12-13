mod lexer;
mod parser;
mod types;
use lexer::*;
use logos::Logos;
use parser::*;
use types::Var;

fn main() {
    let src = "(* (+ 1 2) (- 7.2 -2.0)))";
    let lex = Token::lexer(src);
    let lexed: Vec<_> = lex.collect();

    let var = Var::new(String::from("x"));

    match parse_expressions(&lexed) {
        Some((exprs, _)) => {
            for e in exprs {
                println!("{}", e);
                println!(" => {}", e.derivative(&var));
            }
        }
        None => eprintln!("Failed to parse expressions"),
    }
}
