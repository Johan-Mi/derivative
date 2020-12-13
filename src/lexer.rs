use super::types::Operator;
use logos::Logos;

#[derive(Debug, Logos)]
pub enum Token {
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex(r"[+\-*/]", |lex| lex.slice().parse())]
    Operator(Operator),

    #[regex(r"-?[0-9]+", |lex| lex.slice().parse())]
    Int(i32),
    #[regex(r"-?[0-9]+\.[0-9]+", |lex| lex.slice().parse())]
    Float(f32),

    #[error]
    #[regex(r"\s+", logos::skip)]
    Error,
}
