#[macro_use]
extern crate lalrpop_util;

mod ast;
lalrpop_mod!(pub kb);

use ast::*;

#[test]
fn expression_test() {
    let expr = kb::ExprParser::new()
        .parse("22 * 44 + 66")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
}

fn main() {
    println!("Hello, world!");
}
