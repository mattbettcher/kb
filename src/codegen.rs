
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
                    Function(_, _) => println!("Can't define a function inside a function!"),
                    Expr(expr) => {
                        expr_gen(&expr, code);
                    }
                }
            }
        },
        Expr(_) => println!("Cannot have free standing expressions."),
    }
    code.cur_reg = 0;
    code.text.push_str("\tPUSH EAX\n");
    code.text.push_str("\tHLT\n");          // TEMP
}

fn expr_gen(e: &Expr, code: &mut Code) {
    use crate::ast::Expr::*;
    use crate::ast::Opcode::*;
    match e {
        Number(n) => { 
            code.text.push_str(format!("\tMOV {:}, {:}\n", reg(code.cur_reg), n).as_str());
        },
        Op(l, op, r) => {
            expr_gen(&*l, code);
            code.cur_reg = 1;
            expr_gen(&*r, code);
            match op {
                Mul => { code.text.push_str(format!("\tMUL EAX, {:}\n", reg(code.cur_reg)).as_str()); },
                Div => { code.text.push_str(format!("\tDIV EAX, {:}\n", reg(code.cur_reg)).as_str()); },
                Add => { code.text.push_str(format!("\tADD EAX, {:}\n", reg(code.cur_reg)).as_str()); },
                Sub => { code.text.push_str(format!("\tSUB EAX, {:}\n", reg(code.cur_reg)).as_str()); },
            };
            //code.text.push_str("\tPUSH EAX\n");
            //code.cur_reg = 1;
        },
        Error => todo!(),
    }
}

fn reg(index: usize) -> &'static str {
    match index {
        0 => "EAX",
        1 => "EBX",
        2 => "ECX",
        3 => "EDX",
        _ => panic!(),
    }
}