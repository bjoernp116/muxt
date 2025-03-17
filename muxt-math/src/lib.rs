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
        let input = String::from("3 + 5 * 4");
        let mut parser: AstFactory = lexer::analyze(input).unwrap().into();

        let tree = parser.parse_term();
        println!("{:#?}", tree);
    }
}
