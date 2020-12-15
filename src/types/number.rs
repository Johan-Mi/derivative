use super::{Expr, Var};
use derive_more::Display;
use std::ops;

#[derive(Display, Clone, Copy)]
pub enum Number {
    Int(i32),
    Float(f32),
}

impl Number {
    pub fn derivative(&self, _var: &Var) -> Expr {
        Expr::Number(Number::Int(0))
    }

    pub fn simplify(&self) -> Expr {
        Expr::Number(*self)
    }
}

impl PartialEq<i32> for Number {
    fn eq(&self, other: &i32) -> bool {
        use Number::*;
        match *self {
            Int(i) if i == *other => true,
            Float(f) if f == *other as f32 => true,
            _ => false,
        }
    }
}

impl ops::Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        use Number::*;
        match (self, other) {
            (Int(l), Int(r)) => Int(l + r),
            (Int(l), Float(r)) => Float(l as f32 + r),
            (Float(l), Int(r)) => Float(l + r as f32),
            (Float(l), Float(r)) => Float(l + r),
        }
    }
}

impl ops::Sub for Number {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        use Number::*;
        match (self, other) {
            (Int(l), Int(r)) => Int(l - r),
            (Int(l), Float(r)) => Float(l as f32 - r),
            (Float(l), Int(r)) => Float(l - r as f32),
            (Float(l), Float(r)) => Float(l - r),
        }
    }
}

impl ops::Mul for Number {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        use Number::*;
        match (self, other) {
            (Int(l), Int(r)) => Int(l * r),
            (Int(l), Float(r)) => Float(l as f32 * r),
            (Float(l), Int(r)) => Float(l * r as f32),
            (Float(l), Float(r)) => Float(l * r),
        }
    }
}

impl ops::Div for Number {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        use Number::*;
        match (self, other) {
            (Int(l), Int(r)) => Float(l as f32 / r as f32),
            (Int(l), Float(r)) => Float(l as f32 / r),
            (Float(l), Int(r)) => Float(l / r as f32),
            (Float(l), Float(r)) => Float(l / r),
        }
    }
}
