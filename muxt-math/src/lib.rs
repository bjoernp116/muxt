mod lexer;
mod expression;
mod parser;



#[cfg(test)]
mod tests {
    use crate::{lexer::LexicalSequence, parser::AstFactory};

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
        let input = String::from("3 + 5 * (4 + 2) * 3");
        let result: LexicalSequence = lexer::analyze(input).unwrap();

        println!("Lexer output: {:#?}", result);

        let mut parser: AstFactory = result.into();

        let tree = parser.parse_term();
        println!("{:#?}", tree);
    }
}
