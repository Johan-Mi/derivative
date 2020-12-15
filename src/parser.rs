use super::lexer::*;
use super::types::*;
use derive_more::Constructor;
use std::rc::Rc;

fn parse_var(tokens: &[Token]) -> Option<(Var, &[Token])> {
    match tokens.split_first() {
        Some((Token::Ident(symbol), tokens)) => {
            Some((Var::new(symbol.to_string()), tokens))
        }
        _ => None,
    }
}

fn parse_lparen(tokens: &[Token]) -> Option<&[Token]> {
    match tokens.split_first() {
        Some((Token::LParen, tokens)) => Some(tokens),
        _ => None,
    }
}

fn parse_rparen(tokens: &[Token]) -> Option<&[Token]> {
    match tokens.split_first() {
        Some((Token::RParen, tokens)) => Some(tokens),
        _ => None,
    }
}

fn parse_bin_operator(tokens: &[Token]) -> Option<(BinOperator, &[Token])> {
    match tokens.split_first() {
        Some((Token::BinOperator(op), tokens)) => Some((*op, tokens)),
        _ => None,
    }
}

fn parse_number(tokens: &[Token]) -> Option<(Number, &[Token])> {
    match tokens.split_first() {
        Some((Token::Int(i), tokens)) => Some((Number::Int(*i), tokens)),
        Some((Token::Float(f), tokens)) => Some((Number::Float(*f), tokens)),
        _ => None,
    }
}

#[derive(Constructor)]
struct IterParseExpr<'a> {
    pub tokens: &'a [Token],
}

impl<'a> Iterator for IterParseExpr<'a> {
    type Item = Expr;

    fn next(&mut self) -> Option<Self::Item> {
        let (expr, tokens) = parse_expr(self.tokens)?;
        self.tokens = tokens;
        Some(expr)
    }
}

fn parse_bin_operation(tokens: &[Token]) -> Option<(BinOperation, &[Token])> {
    let tokens = parse_lparen(tokens)?;

    let (operator, tokens) = parse_bin_operator(tokens)?;
    let (first_arg, tokens) = parse_expr(tokens)?;
    let (second_arg, tokens) = parse_expr(tokens)?;
    let first_op =
        BinOperation::new(operator, Rc::new(first_arg), Rc::new(second_arg));

    let mut it = IterParseExpr::new(tokens);
    let ret = it.by_ref().fold(first_op, |acc, e| {
        BinOperation::new(
            operator,
            Rc::new(Expr::BinOperation(acc)),
            Rc::new(e),
        )
    });

    let tokens = parse_rparen(it.tokens)?;
    Some((ret, tokens))
}

fn parse_expr(tokens: &[Token]) -> Option<(Expr, &[Token])> {
    if let Some((expr, tokens)) = parse_number(tokens) {
        Some((Expr::Number(expr), tokens))
    } else if let Some((expr, tokens)) = parse_var(tokens) {
        Some((Expr::Var(expr), tokens))
    } else if let Some((expr, tokens)) = parse_bin_operation(tokens) {
        Some((Expr::BinOperation(expr), tokens))
    } else {
        None
    }
}

pub fn parse_expressions(
    mut tokens: &[Token],
) -> Option<(Vec<Expr>, &[Token])> {
    let mut ret = Vec::new();

    while let Some((expr, remaining_tokens)) = parse_expr(tokens) {
        ret.push(expr);
        tokens = remaining_tokens;
    }

    if ret.is_empty() {
        None
    } else {
        Some((ret, tokens))
    }
}
