use anyhow::anyhow;

use crate::parser::{Node, AST};
use crate::parser::Operator;



type Result<T> = anyhow::Result<T>;

pub struct Interpreter {
    pub ast: AST,
}

impl Interpreter {
    pub fn simplify(&mut self) -> Result<Node> {
        let result = Self::simplify_rec(self.ast.head.clone())?;
        self.ast.head = result.clone();
        Ok(result)
    }
    pub fn simplify_rec(node: Node) -> Result<Node> {
        match node {
            Node::Binary { left, operator, right } => {
                let left_simplified  = Self::simplify_rec(*left)?;
                let right_simplified = Self::simplify_rec(*right)?;

                if let (Node::Number(l), Node::Number(r)) = (&left_simplified, &right_simplified) {
                    if let Ok(result) = operator.eval(*l, *r) {
                        return Ok(Node::Number(result));
                    }
                }

                match (&left_simplified, &operator, &right_simplified) {
                    (_, Operator::Add, Node::Number(0.0)) => return Ok(left_simplified),  // x + 0 = x
                    (Node::Number(0.0), Operator::Add, _) => return Ok(right_simplified), // 0 + x = x
                    (_, Operator::Sub, Node::Number(0.0)) => return Ok(left_simplified),  // x - 0 = x
                    (Node::Number(0.0), Operator::Sub, _) => return Ok(right_simplified),  // 0 - x = x
                    (_, Operator::Mul, Node::Number(1.0)) => return Ok(left_simplified),  // x * 1 = x
                    (Node::Number(1.0), Operator::Mul, _) => return Ok(right_simplified), // 1 * x = x
                    (_, Operator::Mul, Node::Number(0.0)) | (Node::Number(0.0), Operator::Mul, _) => return Ok(Node::Number(0.0)), // x * 0 | 0 * x = 0
                    _ => {}
                }

                // Combine like terms (addition and subtraction)
                if let (Node::Variable(l), Node::Variable(r)) = (&left_simplified, &right_simplified) {
                    println!("l: {}, r: {}", l, r);
                    if l == r {
                        if let Operator::Add = operator {
                            return Ok(Node::Binary {
                                left: Box::new(Node::Number(2.0)),
                                operator: Operator::Mul,
                                right: Box::new(Node::Variable(l.clone())),
                            });
                        }
                    }
                }

                Ok(Node::Binary {
                    left: Box::new(left_simplified),
                    operator: operator.clone(),
                    right: Box::new(right_simplified)
                })

            },
            Node::Equation { left, right } => {
                let left_simplified  = Self::simplify_rec(*left)?;
                let right_simplified = Self::simplify_rec(*right)?;


                Ok(Node::Equation {
                    left: Box::new(left_simplified),
                    right: Box::new(right_simplified)
                })
            }
            _ => Ok(node.clone())
        }
    }

    pub fn solve_for(&mut self, var: char) -> Result<Node> {
        if let Node::Equation { left, right } = self.ast.head.clone() {
            let mut left_simplified: Node = Self::simplify_rec(*left)?;
            let mut right_simplified: Node = Self::simplify_rec(*right)?;

            if Self::find_var(right_simplified.clone(), var) {
                let temp = left_simplified.clone();
                left_simplified = right_simplified.clone();
                right_simplified = temp;

            }

            let result = match (left_simplified.clone(), right_simplified.clone()) {
                (Node::Binary { left, operator, right }, rhs) => {
                    Self::rearrange_terms(*left, operator, *right, rhs, var)
                },
                _ => todo!()
            };
            if let Ok(equation) = result {
                self.ast.head = equation.clone();
                return Ok(equation);
            } else {
                return Err(anyhow!("Failed to solve for variable {:#?}", var));
            }
        } else {
            return Err(anyhow!("The input {:#?}, is not an equation", self.ast.head.clone()));
        }
    }

    fn rearrange_terms(
        left: Node,
        operator: Operator,
        right: Node,
        rhs: Node,
        var: char
    ) -> Result<Node> {
        if let Node::Variable(v) = right {
            if v == var {
                return Ok(Node::Equation {
                    left: Box::new(right),
                    right: Box::new(Node::Binary {
                        left: Box::new(rhs),
                        operator: operator.inverse(),
                        right: Box::new(left)
                    })
                });
            }
        }
        if let Node::Variable(v) = left {
            if v == var {
                return Ok(Node::Equation {
                    left: Box::new(left),
                    right: Box::new(Node::Binary {
                        left: Box::new(rhs),
                        operator: operator.inverse(),
                        right: Box::new(right)
                    })
                })
            }
        }
        Err(anyhow!("Can only rearrange variables!"))
    }
    fn find_var(node: Node, var: char) -> bool {
        match node {
            Node::Variable(v) => v == var,
            Node::Binary { left, right, .. } => {
                Self::find_var(*left, var) || Self::find_var(*right, var)
            }
            _ => false
        }
    }
}


