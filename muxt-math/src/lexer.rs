use anyhow::anyhow;
use std::collections::VecDeque;

#[derive(Clone)]
pub enum LexicalSymbol {
    Varible(char),
    Plus,
    Minus,
    Div,
    Mult,
    Eqal,
    Number(usize),
    OpenParen,
    CloseParen,
}

#[derive(Clone)]
pub struct LexicalExpression {
    pub symbol: LexicalSymbol,
    pub pos: usize,
}

#[derive(Clone)]
pub struct LexicalSequence(pub VecDeque<LexicalExpression>);

pub fn analyze(input: String) -> anyhow::Result<LexicalSequence>{
    let mut buffer: String = String::new();
    let mut out: VecDeque<LexicalExpression> = VecDeque::new();

    for (i, c) in input.chars().enumerate() {
        if c.is_digit(10) {
            buffer.push(c);
        } else {
            if buffer.len() != 0 {
                let expr = LexicalExpression {
                    symbol: LexicalSymbol::Number(usize::from_str_radix(buffer.as_str(), 10).unwrap()),
                    pos: i
                };
                out.push_back(expr);
                buffer.clear();
            }
            let symbol: LexicalSymbol = match c {
                '+' => LexicalSymbol::Plus,
                '-' => LexicalSymbol::Minus,
                '/' => LexicalSymbol::Div,
                '*' => LexicalSymbol::Mult,
                '(' => LexicalSymbol::OpenParen,
                ')' => LexicalSymbol::CloseParen,
                '=' => LexicalSymbol::Eqal,
                ' ' => continue,
                x if x.is_alphabetic() => LexicalSymbol::Varible(x),
                _ => { return Err(anyhow!("Unexpected symbol: {}", c)); }
            };
            let expr = LexicalExpression {
                symbol,
                pos: i,
            };
            out.push_back(expr);
        }

    }

    Ok(LexicalSequence(out))
}

impl std::fmt::Display for LexicalSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use LexicalSymbol::*;
        match self {
            Varible(x) => write!(f, "Var({})", x)?,
            Plus => write!(f, "+")?,
            Minus => write!(f, "-")?,
            Div => write!(f, "/")?,
            Mult => write!(f, "*")?,
            Number(x) => write!(f, "{}", x)?,
            OpenParen => write!(f, "(")?,
            CloseParen => write!(f, ")")?,
            Eqal => write!(f, "=")?,
        };
        Ok(())
    }
}

impl std::fmt::Display for LexicalSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for expr in &self.0 {
            write!(f, "{} ", expr.symbol);
        }
        Ok(())
    }
}



















