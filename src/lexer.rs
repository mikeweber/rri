pub mod token;

use token::{ TokenType, Token };

struct Lexer {
    body:     &'static str,
    pos:      usize,
    read_pos: usize,
    ch:       char
}

impl Lexer {
    pub fn new(body: &'static str) -> Self {
        let mut l = Self {
            body: body,
            pos: 0,
            read_pos: 0,
            ch: ' '
        };
        l.read_char();
        return l;
    }

    pub fn read_char(&mut self) {
        if self.read_pos >= self.body.len() {
            self.ch = 0x00_u8.into();
        } else {
            self.ch = self.body.as_bytes()[self.read_pos] as char;
        }
        self.pos = self.read_pos;
        self.read_pos += 1;
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.pos >= self.body.len() { return None; }

        let tok = match self.ch {
            '=' => Token::new(TokenType::ASSIGN,    self.ch.to_string()),
            ';' => Token::new(TokenType::SEMICOLON, self.ch.to_string()),
            '(' => Token::new(TokenType::LPAREN,    self.ch.to_string()),
            ')' => Token::new(TokenType::RPAREN,    self.ch.to_string()),
            ',' => Token::new(TokenType::COMMA,     self.ch.to_string()),
            '+' => Token::new(TokenType::PLUS,      self.ch.to_string()),
            '{' => Token::new(TokenType::LBRACE,    self.ch.to_string()),
            '}' => Token::new(TokenType::RBRACE,    self.ch.to_string()),
            _   => Token::new(TokenType::EOF,      "\u{0}".to_string()),
        };
        self.read_char();
        return Some(tok);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_parse_a_single_line() {
        let input = "=+(){},;";
        let expected_tokens = vec![
            Token::new(TokenType::ASSIGN,     "=".to_string()),
            Token::new(TokenType::PLUS,       "+".to_string()),
            Token::new(TokenType::LPAREN,     "(".to_string()),
            Token::new(TokenType::RPAREN,     ")".to_string()),
            Token::new(TokenType::LBRACE,     "{".to_string()),
            Token::new(TokenType::RBRACE,     "}".to_string()),
            Token::new(TokenType::COMMA,      ",".to_string()),
            Token::new(TokenType::SEMICOLON,  ";".to_string()),
            Token::new(TokenType::EOF,        "\u{0}".to_string()),
        ];
        let mut lexer = Lexer::new(input);
        for t in expected_tokens {
            let next_token = lexer.next();

            match next_token {
                Some(tok) => {
                    println!("Compare {} and {}", tok.literal, t.literal);
                    assert_eq!(tok.token_type, t.token_type);
                    assert_eq!(tok.literal, t.literal);
                },
                None => {}
            }
        }
    }

    #[test]
    fn it_can_parse_multiple_lines() {
        let input = "five = 5
ten = 10
def add(x, y)
  x + y;
end
result = add five, ten";

        let expected_tokens = vec![
            Token::new(TokenType::IDENT,      "five".to_string()),
            Token::new(TokenType::ASSIGN,     "=".to_string()),
            Token::new(TokenType::INT,        "5".to_string()),
            Token::new(TokenType::IDENT,      "ten".to_string()),
            Token::new(TokenType::ASSIGN,     "=".to_string()),
            Token::new(TokenType::INT,        "10".to_string()),
            Token::new(TokenType::DEF,        "def".to_string()),
            Token::new(TokenType::IDENT,      "add".to_string()),
            Token::new(TokenType::LPAREN,     "(".to_string()),
            Token::new(TokenType::IDENT,      "x".to_string()),
            Token::new(TokenType::COMMA,      ",".to_string()),
            Token::new(TokenType::IDENT,      "y".to_string()),
            Token::new(TokenType::RPAREN,     ")".to_string()),
            Token::new(TokenType::IDENT,      "x".to_string()),
            Token::new(TokenType::PLUS,       "+".to_string()),
            Token::new(TokenType::IDENT,      "y".to_string()),
            Token::new(TokenType::SEMICOLON,  ";".to_string()),
            Token::new(TokenType::END,        "end".to_string()),
            Token::new(TokenType::IDENT,      "result".to_string()),
            Token::new(TokenType::ASSIGN,     "=".to_string()),
            Token::new(TokenType::IDENT,      "add".to_string()),
            Token::new(TokenType::IDENT,      "five".to_string()),
            Token::new(TokenType::COMMA,      ",".to_string()),
            Token::new(TokenType::IDENT,      "ten".to_string()),
        ];

        let mut lexer = Lexer::new(input);
        for t in expected_tokens {
            let next_token = lexer.next();

            match next_token {
                Some(tok) => {
                    println!("Compare {} and {}", tok.literal, t.literal);
                    assert_eq!(tok.token_type, t.token_type);
                    assert_eq!(tok.literal, t.literal);
                },
                None => {}
            }
        }
    }
}
