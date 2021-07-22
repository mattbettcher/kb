
use crate::ast::{Expr, Statement};

pub struct Symbol {
    name: String,
    address: usize,
}

pub struct Code {
    //symbols: Vec<Symbol>,
    pub text: String,
    pub cur_reg: usize,
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
    code.text.push_str("\tret\n");
}

fn expr_gen(e: &Expr, code: &mut Code) {
    use crate::ast::Expr::*;
    use crate::ast::Opcode::*;
    match e {
        Number(n) => { 
            code.text.push_str(format!("\tmov {:}, {:}\n", n, reg(code.cur_reg)).as_str());
            code.cur_reg += 1;
        },
        Op(l, op, r) => {
            expr_gen(&*l, code);
            expr_gen(&*r, code);
            match op {
                Mul => { code.text.push_str(format!("\tmul rbx, rax\n").as_str()); },
                Div => { code.text.push_str(format!("\tdiv rbx, rax\n").as_str()); },
                Add => { code.text.push_str(format!("\tadd rbx, rax\n").as_str()); },
                Sub => { code.text.push_str(format!("\tsub rbx, rax\n").as_str()); },
            };
            code.cur_reg = 0;
        },
        Error => todo!(),
    }
}

fn reg(index: usize) -> &'static str {
    match index {
        0 => "rax",
        1 => "rbx",
        2 => "rcx",
        _ => panic!(),
    }
}