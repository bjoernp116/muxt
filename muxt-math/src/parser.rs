use anyhow::anyhow;

use crate::lexer::{LexicalExpression, LexicalSequence, LexicalSymbol};
use std::collections::VecDeque;

type Result<T> = anyhow::Result<T>;

#[derive(Debug)]
pub enum Node {
    Number(usize),
    Binary {
        left: Box<Node>,
        operator: Operator,
        right: Box<Node>
    },
    Unary {
        child: Box<Node>,
        operator: Operator
    }
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Div,
    Mult,
    Pow
}

pub struct AstFactory {
    tokens: VecDeque<LexicalExpression>,
    current: usize,
}

impl AstFactory {
    pub fn parse_term(&mut self) -> Result<Node> {
        println!("parse_term: {}", self.tokens[self.current].symbol);
        let mut node: Node = self.parse_factor()?;
        while self.current < self.tokens.len() {
            match self.tokens[self.current].symbol {
                LexicalSymbol::Plus | LexicalSymbol::Minus => {
                    let op = self.tokens[self.current].symbol.clone();
                    self.current += 1; // Increment here after capturing the operator
                    if self.current >= self.tokens.len() {
                        break;
                    }
                    let right = Box::new(self.parse_factor()?);
                    node = Node::Binary { left: Box::new(node), operator: to_operator(op), right };
                }
                _ => break
            }
        }
        Ok(node)
    }

    fn parse_factor(&mut self) -> Result<Node> {
        println!("parse_factor: {}", self.tokens[self.current].symbol);
        let mut node: Node = self.parse_exponent()?;
        while self.current < self.tokens.len() {
            match self.tokens[self.current].symbol {
                LexicalSymbol::Mult | LexicalSymbol::Div => {
                    let op = self.tokens[self.current].symbol.clone();
                    self.current += 1; // Increment here after capturing the operator
                    if self.current >= self.tokens.len() {
                        break;
                    }
                    let right = Box::new(self.parse_exponent()?);
                    node = Node::Binary { left: Box::new(node), operator: to_operator(op), right };
                }
                _ => break
            }
        }
        Ok(node)
    }

    fn parse_exponent(&mut self) -> Result<Node> {
        println!("parse_exponent: {}", self.tokens[self.current].symbol);
        let mut node: Node = self.parse_primary()?;
        while self.current < self.tokens.len() {
            match self.tokens[self.current].symbol {
                LexicalSymbol::Pow => {
                    let op = self.tokens[self.current].symbol.clone();
                    self.current += 1; // Increment here after capturing the operator
                    if self.current >= self.tokens.len() {
                        break;
                    }
                    let right = Box::new(self.parse_primary()?);
                    node = Node::Binary { left: Box::new(node), operator: to_operator(op), right };
                }
                _ => break
            }
        }
        Ok(node)
    }

    fn parse_primary(&mut self) -> Result<Node> {
        if self.current >= self.tokens.len() {
            return Err(anyhow!("Out of bounds access in parse_primary"));
        }
        println!("parse_primary: {}", self.tokens[self.current].symbol);
        match self.tokens[self.current].symbol {
            LexicalSymbol::OpenParen => self.parse_paren(),
            _ => self.parse_number()
        }
    }

    fn parse_paren(&mut self) -> Result<Node> {
        println!("parse_paren: {}", self.tokens[self.current].symbol);
        let mut open_p = 0;
        let mut private_tokens: VecDeque<LexicalExpression> = VecDeque::new();

        if self.tokens.len() == 0 {
            return Ok(Node::Number(0));
        }
        match self.tokens[self.current].symbol {
            LexicalSymbol::OpenParen => {},
            _ => { return self.parse_number(); },
        };
        self.current += 1;
        open_p += 1;

        while self.current < self.tokens.len() && open_p != 0 {
            match self.tokens[self.current].symbol {
                LexicalSymbol::OpenParen => open_p += 1,
                LexicalSymbol::CloseParen => open_p -= 1,
                _ => ()
            }
            private_tokens.push_back(self.tokens[self.current].clone());
            self.current += 1;
        }
        if open_p != 0 {
            return Err(anyhow!("Mismatched parentheses!"));
        }
        let mut parser = AstFactory { tokens: private_tokens, current: 0 };
        let node = parser.parse_term()?;
        Ok(node)
    }

    fn parse_number(&mut self) -> Result<Node> {
        if self.current >= self.tokens.len() {
            return Err(anyhow!("Out of bounds access in parse_number"));
        }
        println!("parse_number: {}", self.tokens[self.current].symbol);
        match &self.tokens[self.current].symbol {
            LexicalSymbol::Number(x) => {
                let number = *x; // Capture the number before incrementing
                self.current += 1;
                Ok(Node::Number(number))
            },
            _ => Err(anyhow!("The token {} could not be parsed as a number!", self.tokens[self.current].symbol.clone()))
        }
    }
}

fn to_operator(symbol: LexicalSymbol) -> Operator {
    match symbol {
        LexicalSymbol::Plus => Operator::Plus,
        LexicalSymbol::Minus => Operator::Minus,
        LexicalSymbol::Div => Operator::Div,
        LexicalSymbol::Mult => Operator::Mult,
        LexicalSymbol::Pow => Operator::Pow,
        _ => todo!()
    }
}

impl From<LexicalSequence> for AstFactory {
    fn from(value: LexicalSequence) -> AstFactory {
        AstFactory {
            tokens: value.0,
            current: 0,
        }
    }
}
