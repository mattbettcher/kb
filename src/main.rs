#[macro_use]
extern crate lalrpop_util;

mod ast;
lalrpop_mod!(pub kb);

use petgraph::dot::{Config, Dot};
use petgraph::graph::{Graph, NodeIndex};
use ast::*;
use ast::Opcode::*;

#[test]
fn expression_test() {
    let expr = kb::ExprParser::new()
        .parse("22 * 44 + 66 / 2")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "((22 * 44) + (66 / 2))");
}

fn main() {
    let expr = kb::ExprParser::new()
        .parse("22 * 44 + 66 / 2")
        .unwrap();

    let mut graph = Graph::<String, ()>::new(); // directed and unlabeled
    print_expr_graph(&mut graph, &expr, 0);

    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
}

fn print_expr_graph(graph: &mut Graph::<String, ()>, e: &Expr, indent: usize) -> NodeIndex {
    match e {
        Expr::Number(n) => graph.add_node(n.to_string()),
        Expr::Op(a, op, b) => { 
            let node_a = print_expr_graph(graph, a, indent + 1);
            let node = match op {
                Mul => graph.add_node("*".to_string()),
                Div => graph.add_node("/".to_string()),
                Add => graph.add_node("+".to_string()),
                Sub => graph.add_node("-".to_string()),
            };
            let node_b = print_expr_graph(graph, b, indent + 1);
            graph.add_edge(node, node_a, ());
            graph.add_edge(node, node_b, ());
            node
        },
        Expr::Error => todo!(),
    }
}
