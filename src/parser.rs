use crate::lexer::Lexer;
use crate::lexer::token::{ Token, TokenType };
use crate::ast::node::Node;
use crate::ast::expressions::Expression;
use crate::program::Program;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Token,
    peek_token: Option<Token>,
    errors: Vec<String>,
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
            peek_token: peek,
            errors: vec!(),
        }
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    fn peek_error(&mut self, expected_type: TokenType) {
        let msg = match self.peek() {
            Some(token) => format!("expected next token to be {:?}, got {:?} instead", expected_type, token.token_type),
            None => format!("expected next tokent to be {:?}, got EOF instead", expected_type)
        };

        self.errors.push(msg.clone());
    }

    pub fn current(&self) -> Token {
        self.current_token.clone()
    }

    pub fn peek(&self) -> Option<Token> {
        self.peek_token.clone()
    }

    pub fn parse_program(&mut self) -> (Program, Vec<String>) {
        let mut program = Program::new();
        while !self.is_eof() {
            let expr = self.parse_expression();
            match expr {
                Some(expression) => program.push(expression),
                None => ()
            }
            self.next();
        }

        println!("finished parsing program. found {} errors.", self.errors().len());
        return (program, self.errors());
    }

    fn parse_expression(&mut self) -> Option<Expression<'a>> {
        match self.current().token_type {
            TokenType::IDENT  => self.parse_ident_expression(),
            TokenType::INT    => self.parse_integer(),
            TokenType::RETURN => self.parse_return_expression(),
            _ => None
        }
    }

    fn parse_ident_expression(&mut self) -> Option<Expression<'a>> {
        match self.peek() {
            Some(ref peek_token) => {
                match peek_token.token_type {
                    TokenType::ASSIGN => self.parse_assign_expression(),
                    TokenType::SEMICOLON => self.create_ident_expression(),
                    TokenType::EOF => self.create_ident_expression(),
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

    fn create_ident_expression(&mut self) -> Option<Expression<'a>> {
        if !self.cur_token_is(TokenType::IDENT) { return None; }

        let ident = Some(Expression::Identifier(self.current().clone(), Node::Identifier(self.current().clone(), self.current().literal.clone())));
        self.next();
        return ident;
    }

    fn parse_integer(&mut self) -> Option<Expression<'a>> {
        let current = self.current().clone();
        while !self.is_end_of_expression() { self.next(); }

        Some(Expression::Value(current, 0))
    }

    fn parse_return_expression(&mut self) -> Option<Expression<'a>> {
        let return_token = self.current().clone();
        self.next();

        match self.parse_expression() {
            Some(right_expr) => Some(Expression::Return(return_token, Box::new(right_expr))),
            None => None
        }
    }

    fn is_end_of_expression(&mut self) -> bool {
        self.cur_token_is(TokenType::SEMICOLON) || self.cur_token_is(TokenType::NEWLINE) || self.is_eof()
    }

    fn cur_token_is(&mut self, expected_type: TokenType) -> bool {
        self.current().token_type == expected_type
    }

    fn expect_peek(&mut self, expected_type: TokenType) -> bool {
        if self.peek_token_is(expected_type) {
            self.next();
            return true;
        } else {
            self.peek_error(expected_type);
            return false;
        }
    }

    fn peek_token_is(&mut self, expected_type: TokenType) -> bool {
        match self.peek() {
            Some(token) => {
                token.token_type == expected_type
            },
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
    fn should_initialize_with_an_ident_and_eof_token() {
        let mut lexer = Lexer::new("foo".to_string());
        let parser = Parser::new(&mut lexer);
        assert_eq!(parser.current().token_type, TokenType::IDENT);
        assert_eq!(parser.peek().unwrap().token_type, TokenType::EOF);
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
        assert_eq!(parser.peek().unwrap().token_type, TokenType::EOF);

        parser.next();

        assert_eq!(parser.current().token_type, TokenType::EOF);
        assert!(parser.peek().is_none());
    }
}
