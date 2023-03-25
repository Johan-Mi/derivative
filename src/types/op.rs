use super::{Expr, Number, Var};
use derive_more::Constructor;
use std::fmt;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOperator {
    Add,
    Sub,
    Mul,
    Div,
}

impl FromStr for BinOperator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BinOperator::{Add, Div, Mul, Sub};
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
    pub const fn as_str(&self) -> &str {
        use BinOperator::{Add, Div, Mul, Sub};
        match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
        }
    }
}

#[derive(Constructor, Clone)]
pub struct BinOperation {
    operator: BinOperator,
    lhs: Rc<Expr>,
    rhs: Rc<Expr>,
}

impl BinOperation {
    pub fn derivative(&self, var: &Var) -> Expr {
        use BinOperator::{Add, Div, Mul, Sub};
        match self.operator {
            Add | Sub => Expr::BinOperation(Self::new(
                self.operator,
                Rc::new(self.lhs.derivative(var)),
                Rc::new(self.rhs.derivative(var)),
            )),
            Mul => Expr::BinOperation(Self::new(
                Add,
                Rc::new(Expr::BinOperation(Self::new(
                    Mul,
                    Rc::new(self.lhs.derivative(var)),
                    self.rhs.clone(),
                ))),
                Rc::new(Expr::BinOperation(Self::new(
                    Mul,
                    self.lhs.clone(),
                    Rc::new(self.rhs.derivative(var)),
                ))),
            )),
            Div => todo!(),
        }
    }

    pub fn simplify(&self) -> Expr {
        let lhs = self.lhs.simplify();
        let rhs = self.rhs.simplify();
        match (lhs, rhs) {
            (Expr::Number(lhs), Expr::Number(rhs)) => {
                Expr::Number(match self.operator {
                    BinOperator::Add => lhs + rhs,
                    BinOperator::Sub => lhs - rhs,
                    BinOperator::Mul => lhs * rhs,
                    BinOperator::Div => lhs / rhs,
                })
            }
            (Expr::Number(lhs), _rhs) if lhs == 0 => {
                Expr::Number(Number::Int(0))
            }
            (_lhs, Expr::Number(rhs)) if rhs == 0 => {
                Expr::Number(Number::Int(0))
            }
            (Expr::Number(lhs), rhs) if lhs == 1 => rhs,
            (lhs, Expr::Number(rhs)) if rhs == 1 => lhs,
            (lhs, rhs) => Expr::BinOperation(Self::new(
                self.operator,
                Rc::new(lhs),
                Rc::new(rhs),
            )),
        }
    }
}

impl fmt::Display for BinOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn bin_operation_to_string(bop: &BinOperation) -> String {
            match &*bop.lhs {
                Expr::BinOperation(bl) if bl.operator == bop.operator => {
                    format!("{} {}", bin_operation_to_string(bl), bop.rhs)
                }
                _ => {
                    format!("{} {} {}", bop.operator.as_str(), bop.lhs, bop.rhs)
                }
            }
        }
        write!(f, "({})", bin_operation_to_string(self))
    }
}
