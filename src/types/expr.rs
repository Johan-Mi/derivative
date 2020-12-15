use super::{BinOperation, Number, Var};
use derive_more::Display;

#[derive(Display)]
pub enum Expr {
    Number(Number),
    Var(Var),
    BinOperation(BinOperation),
}

impl Expr {
    pub fn derivative(&self, var: &Var) -> Expr {
        use Expr::*;
        match self {
            Number(contained) => contained.derivative(var),
            Var(contained) => contained.derivative(var),
            BinOperation(contained) => contained.derivative(var),
        }
    }

    pub fn simplify(&self) -> Expr {
        use Expr::*;
        match self {
            Number(contained) => contained.simplify(),
            Var(contained) => contained.simplify(),
            BinOperation(contained) => contained.simplify(),
        }
    }
}
