
use std::fmt::{Debug, Error, Formatter};

#[derive(Clone, Debug)]
pub enum Statement {
    Function(String, Vec<(String, String)>, Option<Vec<String>>, Vec<Statement>),
    If(Expr, Vec<Statement>),
    Expr(Expr),
}

#[derive(Clone)]
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Call(String),       // todo: arguments & return
    Error,
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    Equ,
    Neq,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match &self {
            Number(n) => write!(fmt, "{:?}", n),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Call(name) => write!(fmt, "{:}", name),
            Error => write!(fmt, "error"),
        }
    }
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
            Equ => write!(fmt, "=="),
            Neq => write!(fmt, "!="),
        }
    }
}