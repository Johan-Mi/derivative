use super::{BinOperation, Number, Var};
use derive_more::Display;

#[derive(Display)]
pub enum Expr {
    Number(Number),
    Var(Var),
    BinOperation(BinOperation),
}

impl Expr {
    pub fn derivative(&self, var: &Var) -> Self {
        use Expr::{BinOperation, Number, Var};
        match self {
            Number(_contained) => Self::Number(super::Number::Int(0)),
            Var(contained) => contained.derivative(var),
            BinOperation(contained) => contained.derivative(var),
        }
    }

    pub fn simplify(&self) -> Self {
        use Expr::{BinOperation, Number, Var};
        match self {
            Number(contained) => contained.simplify(),
            Var(contained) => contained.simplify(),
            BinOperation(contained) => contained.simplify(),
        }
    }
}
