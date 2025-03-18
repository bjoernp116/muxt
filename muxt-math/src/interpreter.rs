use anyhow::anyhow;

use crate::parser::{Node, AST};
use crate::parser::Operator;



type Result<T> = anyhow::Result<T>;

pub struct Interpreter {
    pub ast: AST,
}

impl Interpreter {
    pub fn evaluate(self) -> Result<Node> {
        //Self::evaluate_expr(self.ast.head)
        Self::simplify(self.ast.head)
    }
    pub fn evaluate_expr(node: Node) -> Result<f32> {
        match node {
            Node::Number(x) => Ok(x),
            Node::Binary { left, operator, right } => {
                let left = Self::evaluate_expr(*left)?;
                let right = Self::evaluate_expr(*right)?;
                let out = operator.eval(left, right)?;
                println!("{} {} {} = {}", left, operator, right, out);
                Ok(out)
            },
            Node::Variable(x) => Err(anyhow!("Variable {}, cant be parsed in an expr!", x)),
            Node::Equation { left, right } => Err(anyhow!("Assignments cant be parsed as expr!"))
        }
    }
    pub fn simplify(node: Node) -> Result<Node> {
        match node {
            Node::Binary { left, operator, right } => {
                let left_simplified  = Self::simplify(*left)?;
                let right_simplified = Self::simplify(*right)?;

                if let (Node::Number(l), Node::Number(r)) = (&left_simplified, &right_simplified) {
                    let result = operator.eval(*l, *r)?;
                    Ok(Node::Number(result))
                } else {
                    Ok(Node::Binary {
                        left: Box::new(left_simplified),
                        operator: operator.clone(),
                        right: Box::new(right_simplified)
                    })
                }
            },
            Node::Equation { left, right } => {
                let left_simplified  = Self::simplify(*left)?;
                let right_simplified = Self::simplify(*right)?;


                Ok(Node::Equation {
                    left: Box::new(left_simplified),
                    right: Box::new(right_simplified)
                })
            }
            _ => Ok(node.clone())
        }
    }
    pub fn solve_for(node: Node, variable: char) -> Result<Node> {
        todo!()
    }
}
