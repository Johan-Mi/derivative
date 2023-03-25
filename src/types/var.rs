use super::{Expr, Number};
use derive_more::{Constructor, Display};

#[derive(Constructor, Display, PartialEq, Eq, Clone)]
pub struct Var {
    name: String,
}

impl Var {
    pub fn derivative(&self, var: &Self) -> Expr {
        if self == var {
            Expr::Number(Number::Int(1))
        } else {
            Expr::Var(self.clone())
        }
    }

    pub fn simplify(&self) -> Expr {
        Expr::Var(self.clone())
    }
}
