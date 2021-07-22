
use crate::ast::{Expr, Statement};

pub struct Symbol {
    name: String,
    address: usize,
}

pub struct Code {
    symbols: Vec<Symbol>,
    text: String,
}

// given a function - generate the assembly
pub fn func_gen(func_stmt: &Statement, code: &mut Code) {
    use crate::ast::Statement::*;
    
    match func_stmt {
        Function(name, stmts) => {
            code.text.push_str(format!("{:}:\n", name).as_str()); // label for function
            for stmt in stmts {
                match stmt {
                    Function(_, _) => println!("Cant't define a function inside a function!"),
                    Expr(expr) => {
                        expr_gen(&expr, code);
                    }
                }
            }
        },
        Expr(_) => println!("Cannot have free standing expressions."),
    }
}

fn expr_gen(e: &Expr, code: &mut Code) {
    use crate::ast::Expr::*;
    use crate::ast::Opcode::*;
    match e {
        Number(n) => *n,
        Op(l, op, r) => {
            expr_gen(&*l, code);
            expr_gen(&*r, code);
            match op {
                Mul => { println!("mul {:}, {:}", ll, rl); ll * rl},
                Div => { println!("div {:}, {:}", ll, rl); ll / rl},
                Add => { println!("sub {:}, {:}", ll, rl); ll + rl},
                Sub => { println!("add {:}, {:}", ll, rl); ll - rl},
            }
        },
        Error => todo!(),
    }
}