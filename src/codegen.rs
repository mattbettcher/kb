
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
        Function(name, _, _, stmts) => {
            code.text.push_str(format!("{:}:\n", name).as_str()); // label for function
            code.text.push_str("\tpush rbp\n");         // save stack frame pointer
            code.text.push_str("\tmov rbp, rsp\n");     // set stack frame pointer
            // here we get params from the stack
            // this should be a loop and will need to handle a bunch of different stuff
            code.text.push_str("\tmov DWORD PTR [rbp-4], edi\n"); // get first 4 byte param

            for stmt in stmts {
                match stmt {
                    Function(_, _, _, _) => println!("Can't define a function inside a function!"),
                    Expr(expr) => {
                        expr_gen(&expr, code);
                    },
                    If(expr, block) => {
                        if_gen(expr, block, code);
                    },
                }
            }
        },
        Expr(_) => println!("Cannot have free standing expressions."),
        If(_, _) => println!("Cannot have free standing expressions."),
    }
    code.cur_reg = 0;
    code.text.push_str("\tPUSH EAX\n");
    code.text.push_str("\tHLT\n");          // TEMP
}

fn if_gen(expr: &Expr, block: &Vec<Statement>, code: &mut Code) -> () {
    // todo: add expression handling, right now there is no operators for this
    code.text.push_str("\tCMP todo\n");
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
                // mul and div are wrong?
                Mul => { code.text.push_str(format!("\tMUL EAX, {:}\n", reg(code.cur_reg)).as_str()); },
                Div => { code.text.push_str(format!("\tDIV EAX, {:}\n", reg(code.cur_reg)).as_str()); },
                Add => { code.text.push_str(format!("\tADD EAX, {:}\n", reg(code.cur_reg)).as_str()); },
                Sub => { code.text.push_str(format!("\tSUB EAX, {:}\n", reg(code.cur_reg)).as_str()); },
                Equ => { code.text.push_str(format!("\tCMP EAX, {:}\n", reg(code.cur_reg)).as_str()); },
                Neq => { code.text.push_str(format!("\tCMP EAX, {:}\n", reg(code.cur_reg)).as_str()); },
            };
        },
        Call(name) => {
            code.text.push_str(format!("\tCALL {:}\n", name).as_str());
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