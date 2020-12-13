use super::{Expr, Var};
use derive_more::Display;

#[derive(Display)]
pub enum Number {
    Int(i32),
    Float(f32),
}

impl Number {
    pub fn derivative(&self, _var: &Var) -> Expr {
        Expr::Number(Number::Int(0))
    }
}
