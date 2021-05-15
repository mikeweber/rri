use crate::lexer::Lexer;
use crate::lexer::token::{ Token, TokenType };
use crate::ast::node::Node;
use crate::ast::expressions::Expression;
use crate::program::Program;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Token,
    peek_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let current = match lexer.next() {
            Some(token) => token,
            None => Token::new(TokenType::EOF, "\u{0}".to_string())
        };
        let peek = lexer.next();
        Self {
            lexer: lexer,
            current_token: current,
            peek_token: peek
        }
    }

    pub fn current(&self) -> Token {
        self.current_token.clone()
    }

    pub fn peek(&self) -> Option<Token> {
        self.peek_token.clone()
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();
        while !self.is_eof() {
            let expr = self.parse_expression();
            match expr {
                Some(expression) => program.push(expression),
                None => ()
            }
            self.next();
        }

        return program
    }

    fn parse_expression(&mut self) -> Option<Expression<'a>> {
        match self.current().token_type {
            TokenType::IDENT  => self.parse_ident_expression(self.current().clone()),
            TokenType::INT    => self.parse_integer(self.current().clone()),
            _ => None
        }
    }

    fn parse_ident_expression(&mut self, token: Token) -> Option<Expression<'a>> {
        match self.peek() {
            Some(peek_token) => {
                match peek_token.token_type {
                    TokenType::ASSIGN => self.parse_assign_expression(),
                    _ => None
                }
            },
            None => None
        }

    }

    fn parse_assign_expression(&mut self) -> Option<Expression<'a>> {
        if !self.cur_token_is(TokenType::IDENT) { return None; }

        let name = Node::Identifier(self.current().clone(), self.current().literal.clone());

        if !self.expect_peek(TokenType::ASSIGN) { return None; }

        self.next();
        match self.parse_expression() {
            Some(right_expr) => Some(Expression::Assign(self.current().clone(), name, Box::new(right_expr))),
            None => None
        }
    }

    fn parse_integer(&mut self, token: Token) -> Option<Expression<'a>> {
        while !self.is_end_of_expression() { self.next(); }
        Some(Expression::Value(token, 0))
    }

    fn is_end_of_expression(&mut self) -> bool {
        self.cur_token_is(TokenType::SEMICOLON) || self.cur_token_is(TokenType::NEWLINE) || self.is_eof()
    }

    fn cur_token_is(&mut self, expected_type: TokenType) -> bool {
        self.current().token_type == expected_type
    }

    fn expect_peek(&mut self, expected_type: TokenType) -> bool {
        if !self.peek_token_is(expected_type) { return false; }

        self.next();
        return true;
    }

    fn peek_token_is(&mut self, expected_type: TokenType) -> bool {
        match self.peek() {
            Some(token) => token.token_type == expected_type,
            None => false
        }
    }

    fn is_eof(&mut self) -> bool {
        self.peek().is_none() || self.peek().unwrap().token_type == TokenType::EOF
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let result = self.peek_token.clone();
        match result.clone() {
            Some(token) => {
                self.current_token = token;
                self.peek_token = self.lexer.next();
                result
            },
            None => None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_initialize_with_eof_when_lexer_is_empty() {
        let mut lexer = Lexer::new("".to_string());
        let parser = Parser::new(&mut lexer);
        assert_eq!(parser.current().token_type, TokenType::EOF);
        assert!(parser.peek().is_none());
    }

    #[test]
    fn should_initialize_with_a_single_token() {
        let mut lexer = Lexer::new("foo".to_string());
        let parser = Parser::new(&mut lexer);
        assert_eq!(parser.current().token_type, TokenType::IDENT);
        assert!(parser.peek().is_none());
    }

    #[test]
    fn should_initialize_with_first_two_tokens() {
        let mut lexer = Lexer::new("foo =".to_string());
        let parser = Parser::new(&mut lexer);
        assert_eq!(parser.current().token_type, TokenType::IDENT);
        assert_eq!(parser.peek().unwrap().token_type, TokenType::ASSIGN);
    }

    #[test]
    fn should_be_able_to_iterate_through_tokens() {
        let mut lexer = Lexer::new("foo = 5;".to_string());
        let mut parser = Parser::new(&mut lexer);
        assert_eq!(parser.current().token_type, TokenType::IDENT);
        assert_eq!(parser.peek().unwrap().token_type, TokenType::ASSIGN);

        parser.next();

        assert_eq!(parser.current().token_type, TokenType::ASSIGN);
        assert_eq!(parser.peek().unwrap().token_type, TokenType::INT);

        parser.next();

        assert_eq!(parser.current().token_type, TokenType::INT);
        assert_eq!(parser.peek().unwrap().token_type, TokenType::SEMICOLON);

        parser.next();

        assert_eq!(parser.current().token_type, TokenType::SEMICOLON);
        assert!(parser.peek().is_none());
    }
}
