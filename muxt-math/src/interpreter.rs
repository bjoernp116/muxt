use anyhow::anyhow;

use crate::parser::{Node, AST};
use crate::parser::Operator;



type Result<T> = anyhow::Result<T>;

pub struct Interpreter {
    pub ast: AST,
}

impl Interpreter {
    pub fn evaluate(self) -> Result<i32> {
        if self.ast.vars == 0 {
            return Self::evaluate_expr(self.ast.head);
        }
        Self::evaluate_expr_var(self.ast.head)
    }
    pub fn evaluate_expr(node: Node) -> Result<i32> {
        match node {
            Node::Number(x) => Ok(x as i32),
            Node::Binary { left, operator, right } => {
                let left = Self::evaluate_expr(*left)?;
                let right = Self::evaluate_expr(*right)?;
                let out = operator.eval(left, right)?;
                println!("{} {} {} = {}", left, operator, right, out);
                Ok(out)
            },
            Node::Variable(x) => Err(anyhow!("Variable {}, cant be parsed in an expr!", x))
        }
    }

    pub fn evaluate_expr_var(node: Node) -> Result<i32> {
        todo!()
    }
}
