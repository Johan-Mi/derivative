use super::{Expr, Var};
use derive_more::{Constructor, Display};
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum BinOperator {
    Add,
    Sub,
    Mul,
    Div,
}

impl FromStr for BinOperator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BinOperator::*;
        match s {
            "+" => Ok(Add),
            "-" => Ok(Sub),
            "*" => Ok(Mul),
            "/" => Ok(Div),
            _ => Err(()),
        }
    }
}

impl BinOperator {
    pub fn as_str(&self) -> &str {
        use BinOperator::*;
        match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
        }
    }
}

#[derive(Constructor, Display)]
#[display(fmt = "({} {} {})", "operator.as_str()", lhs, rhs)]
pub struct BinOperation {
    operator: BinOperator,
    lhs: Rc<Expr>,
    rhs: Rc<Expr>,
}

impl BinOperation {
    pub fn derivative(&self, var: &Var) -> Expr {
        use BinOperator::*;
        match self.operator {
            Add | Sub => Expr::BinOperation(BinOperation::new(
                self.operator,
                Rc::new(self.lhs.derivative(var)),
                Rc::new(self.rhs.derivative(var)),
            )),
            Mul => Expr::BinOperation(BinOperation::new(
                Add,
                Rc::new(Expr::BinOperation(BinOperation::new(
                    Mul,
                    Rc::new(self.lhs.derivative(var)),
                    self.rhs.clone(),
                ))),
                Rc::new(Expr::BinOperation(BinOperation::new(
                    Mul,
                    self.lhs.clone(),
                    Rc::new(self.rhs.derivative(var)),
                ))),
            )),
            _ => todo!(),
        }
    }
}
