use super::{Application, Number, Var};
use derive_more::Display;

#[derive(Display)]
pub enum Expr {
    Number(Number),
    Var(Var),
    Application(Application),
}

impl Expr {
    pub fn derivative(&self, var: &Var) -> Expr {
        match self {
            Expr::Number(contained) => contained.derivative(var),
            Expr::Var(contained) => contained.derivative(var),
            Expr::Application(contained) => contained.derivative(var),
        }
    }
}
