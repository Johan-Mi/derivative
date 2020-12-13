use super::lexer::*;
use super::types::*;

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

fn parse_operator(tokens: &[Token]) -> Option<(Operator, &[Token])> {
    match tokens.split_first() {
        Some((Token::Operator(op), tokens)) => Some((*op, tokens)),
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

fn parse_application(tokens: &[Token]) -> Option<(Application, &[Token])> {
    let tokens = parse_lparen(tokens)?;
    let (operator, mut tokens) = parse_operator(tokens)?;
    let mut args = Vec::new();
    while let Some((arg, remaining_tokens)) = parse_expression(tokens) {
        tokens = remaining_tokens;
        args.push(arg);
    }
    let tokens = parse_rparen(tokens)?;
    Some((Application::new(operator, args), tokens))
}

fn parse_expression(tokens: &[Token]) -> Option<(Expr, &[Token])> {
    if let Some((expr, unconsumed_tokens)) = parse_number(tokens) {
        return Some((Expr::Number(expr), unconsumed_tokens));
    }
    if let Some((expr, unconsumed_tokens)) = parse_var(tokens) {
        return Some((Expr::Var(expr), unconsumed_tokens));
    }
    if let Some((expr, unconsumed_tokens)) = parse_application(tokens) {
        return Some((Expr::Application(expr), unconsumed_tokens));
    }

    None
}

pub fn parse_expressions(
    mut tokens: &[Token],
) -> Option<(Vec<Expr>, &[Token])> {
    let mut ret = Vec::new();

    while let Some((expr, remaining_tokens)) = parse_expression(tokens) {
        ret.push(expr);
        tokens = remaining_tokens;
    }

    if ret.is_empty() {
        None
    } else {
        Some((ret, tokens))
    }
}
