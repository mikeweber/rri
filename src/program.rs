use crate::ast::node::Node;
use crate::ast::expressions::Expression;

pub struct Program<'a> {
    expressions: Vec<Expression<'a>>
}

impl<'a> Program<'a> {
    pub fn new() -> Program<'a> {
        Program { expressions: vec!() }
    }

    pub fn push(&mut self, expression: Expression<'a>) {
        self.expressions.push(expression)
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use super::*;

    #[test]
    fn should_parse_assign_expressions() {
        let input = "x = 5;y = 10
            foobar = 838383;";
        let mut lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program();
        assert_eq!(program.expressions.len(), 3);
        test_assign_expression(program.expressions[0].clone(), "x".to_string());
        test_assign_expression(program.expressions[1].clone(), "y".to_string());
        test_assign_expression(program.expressions[2].clone(), "foobar".to_string());
    }

    fn test_assign_expression(e: Expression, expected_name: String) {
        match e {
            Expression::Assign(_, identifier, _) => {
                match identifier {
                    Node::Identifier(_, name) => assert_eq!(name, expected_name),
                    _ => panic!("Right expression type, wrong Node type")
                }
            },
            Expression::Value(token, _) => panic!("Wrong expression type, got {:?}", token.token_type)
        }
    }
}
