use super::Operator;
use super::{Expr, Var};
use derive_more::Constructor;
use itertools::Itertools;
use std::fmt::{self, Display};

#[derive(Constructor)]
pub struct Application {
    operator: Operator,
    args: Vec<Expr>,
}

impl Display for Application {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({} {})",
            self.operator,
            self.args
                .iter()
                .map(|e| e.to_string())
                .intersperse(String::from(" "))
                .collect::<String>()
        )
    }
}

impl Application {
    pub fn derivative(&self, var: &Var) -> Expr {
        match self.operator {
            Operator::Add => Expr::Application(Application::new(
                Operator::Add,
                self.args.iter().map(|e| e.derivative(var)).collect(),
            )),
            _ => todo!(),
        }
    }
}
