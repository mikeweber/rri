use crate::lexer::Lexer;
use crate::lexer::token::{ Token, TokenType };

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
