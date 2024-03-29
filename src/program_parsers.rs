use nom::types::CompleteStr;

use crate::expression_parsers::*;
use crate::tokens::Token;

named!(pub program<CompleteStr, Token>,
    ws!(
        do_parse!(
            expressions: many1!(expression) >>
            (
                Token::Program {
                    expressions: expressions
                }
            )
        )
    )
);

mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        let test_program = CompleteStr("1+2");
        let result = program(test_program);
        assert_eq!(result.is_ok(), true);
        let (_, tree) = result.unwrap();
        // TODO: More testws here on the tree
    }
}
