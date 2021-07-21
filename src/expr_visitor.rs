use crate::ast::Expr;

pub fn visit_expr(e: &Expr) -> i32 {
    use crate::ast::Expr::*;
    use crate::ast::Opcode::*;
    match e {
        Number(n) => *n,
        Op(l, op, r) => {
            let ll = visit_expr(&*l);
            let rl = visit_expr(&*r);
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