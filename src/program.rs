use crate::ast::node::Node;
use crate::ast::expressions::Expression;
use crate::lexer::token::{ Token, TokenType };

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

    pub fn to_s(&self) -> String {
        let mut string = String::from("");

        for expr in self.expressions.iter() {
            string = string + &expr.to_s();
        }

        return string;
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
        let (program, errors) = parser.parse_program();
        check_parser_errors(errors);

        assert_eq!(program.expressions.len(), 3);
        test_assign_expression(program.expressions[0].clone(), "x".to_string());
        test_assign_expression(program.expressions[1].clone(), "y".to_string());
        test_assign_expression(program.expressions[2].clone(), "foobar".to_string());
    }

    #[test]
    fn should_parse_return_expressions() {
        let input = "
            return 5;
            return 10
            return 993322
        ";

        let mut lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(&mut lexer);
        let (program, errors) = parser.parse_program();
        check_parser_errors(errors);

        assert_eq!(program.expressions.len(), 3);
        for expr in program.expressions.iter() {
            println!("Checking expression {:?}", expr);
            match expr {
                Expression::Return(token, _) => assert_eq!(token.literal, "return"),
                Expression::Assign(_, _, _) => panic!("expected Return, got Assign"),
                Expression::Value(_, _) => panic!("expected Return, got Value"),
            }
        }
    }

    #[test]
    fn should_print_a_program() {
        let mut program = Program::new();
        let assign_expression = Expression::Assign(
            Token::new(TokenType::ASSIGN, "=".to_string()),
            Node::Identifier(
                Token::new(TokenType::IDENT, "my_var".to_string()),
                "my_var".to_string()
            ),
            Box::new(Expression::Value(
                Token::new(TokenType::IDENT, "another_var".to_string()),
                5
            ))

        );
        program.push(assign_expression);
        assert_eq!(program.to_s(), "my_var = another_var\n");
    }

    fn test_assign_expression(e: Expression, expected_name: String) {
        match e {
            Expression::Assign(_, identifier, _) => {
                match identifier {
                    Node::Identifier(_, name) => assert_eq!(name, expected_name),
                    _ => panic!("Right expression type, wrong Node type")
                }
            },
            Expression::Value(_, _) => panic!("expected Assign, got Value"),
            Expression::Return(_, _) => panic!("expected Assign, got Return"),
        }
    }

    fn check_parser_errors(errors: Vec<String>) {
        if errors.len() == 0 { return; }

        println!("parser has {} errors", errors.len());
        for msg in errors.iter() {
            println!("- {}", msg);
        }
        panic!("end of parser errors");
    }
}
