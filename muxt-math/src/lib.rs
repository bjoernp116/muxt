mod lexer;
mod expression;
mod parser;
mod interpreter;



#[cfg(test)]
mod tests {
    use crate::{interpreter::Interpreter, lexer::LexicalSequence, parser::{AstFactory, AST}};

    use super::*;

    #[test]
    fn lexer() {
        let input = String::from("x + 32+5/ 3 = ) / (");
        let result: LexicalSequence = lexer::analyze(input).unwrap();

        print!("{}", result);

        assert_eq!(format!("{}", result), "Var(x) + 32 + 5 / 3 = ) / ( ".to_owned());
    }

    #[test]
    fn ast() {
        let input = String::from("3 ^ 2 + 5 * (4 + x) * 3");
        let result: LexicalSequence = lexer::analyze(input).unwrap();

        println!("Lexer output: {:#?}", result);

        let mut parser: AstFactory = result.into();

        let tree = parser.parse_term();
        println!("{:#?}", tree);
    }

    #[test]
    fn interpreter_expr() {
        let input = String::from("3 ^ 2 + 5 * (4 + 5) * 3");
        let result: LexicalSequence = lexer::analyze(input).unwrap();

        let mut parser: AstFactory = result.into();
        let ast: AST = parser.parse().unwrap();
        println!("{:#?}", ast.clone());

        let interpreter = Interpreter { ast };
        println!("Output: {:#?}", interpreter.evaluate().unwrap());
    }

    #[test]
    fn simplify() {
        let input = String::from("4 * 2 / (3 + x) = (4 * 3) / 2");
        let result: LexicalSequence = lexer::analyze(input).unwrap();

        let mut parser: AstFactory = result.into();
        let ast: AST = parser.parse().unwrap();
        println!("{:#?}", ast.clone());

        let interpreter = Interpreter { ast };
        println!("Output: {:#?}", interpreter.evaluate().unwrap());
    }
}
