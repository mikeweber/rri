pub mod token;

use token::{ TokenType, Token, KEYWORDS };

pub struct Lexer {
    body:     String,
    pos:      usize,
    read_pos: usize,
    ch:       char
}

impl Lexer {
    pub fn new(body: String) -> Self {
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

    pub fn peek_char(&mut self) -> char {
        let bytes = self.body.as_bytes();
        if self.read_pos >= bytes.len() { return '\u{0}'; }

        bytes[self.read_pos] as char
    }

    pub fn read_identifier(&mut self) -> String {
        let pos = self.pos;
        while self.is_letter(self.ch) {
            self.read_char();
        }
        return std::str::from_utf8(&self.body.as_bytes()[pos..self.pos]).unwrap().to_string();
    }

    pub fn read_number(&mut self) -> String {
        let pos = self.pos;
        while self.is_digit(self.ch) {
            self.read_char();
        }

        return std::str::from_utf8(&self.body.as_bytes()[pos..self.pos]).unwrap().to_string();
    }

    pub fn is_letter(&mut self, ch: char) -> bool {
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_' || ch == '!' || ch == '?'
    }

    pub fn is_digit(&mut self, ch: char) -> bool {
        '0' <= ch && ch <= '9'
    }

    pub fn lookup_ident(&mut self, ident: String) -> TokenType {
        match KEYWORDS.get(&ident[..]) {
            Some(tt) => *tt,
            None => TokenType::IDENT
        }
    }

    pub fn skip_whitespace(&mut self) {
        while self.pos_at_whitespace() {
            self.read_char()
        }
    }

    fn pos_at_whitespace(&mut self) -> bool {
        self.ch == ' ' || self.ch == '\t'
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.pos == self.body.len() {
            self.pos += 1;
            return Some(Token::new(TokenType::EOF, "\u{0}".to_string()));
        }
        if self.pos >= self.body.len() { return None; }

        self.skip_whitespace();

        let tok = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::EQ,        "==".to_string())
                } else {
                    Token::new(TokenType::ASSIGN,    self.ch.to_string())
                }
            },
            '+' => Token::new(TokenType::PLUS,      self.ch.to_string()),
            '-' => Token::new(TokenType::MINUS,     self.ch.to_string()),
            '!' => {
                if self.peek_char() == '=' {
                    let mut ch = String::from(self.ch);
                    self.read_char();
                    ch.push(self.ch);
                    Token::new(TokenType::NOTEQ,   ch)
                } else {
                    Token::new(TokenType::BANG,      self.ch.to_string())
                }
            }
            '/' => Token::new(TokenType::SLASH,     self.ch.to_string()),
            '*' => Token::new(TokenType::ASTERISK,  self.ch.to_string()),
            '<' => Token::new(TokenType::LT,        self.ch.to_string()),
            '>' => Token::new(TokenType::GT,        self.ch.to_string()),
            ';' => Token::new(TokenType::SEMICOLON, self.ch.to_string()),
            ',' => Token::new(TokenType::COMMA,     self.ch.to_string()),
            '(' => Token::new(TokenType::LPAREN,    self.ch.to_string()),
            ')' => Token::new(TokenType::RPAREN,    self.ch.to_string()),
            '{' => Token::new(TokenType::LBRACE,    self.ch.to_string()),
            '}' => Token::new(TokenType::RBRACE,    self.ch.to_string()),
            '\n' => Token::new(TokenType::NEWLINE,  self.ch.to_string()),
            '\r' => Token::new(TokenType::NEWLINE,  self.ch.to_string()),
            _ => {
                if self.is_letter(self.ch) {
                    let literal = self.read_identifier();
                    return Some(Token::new(self.lookup_ident(literal.clone()), literal));
                } else if self.is_digit(self.ch) {
                    return Some(Token::new(TokenType::INT, self.read_number()));
                } else {
                    Token::new(TokenType::ILLEGAL, self.ch.to_string())
                }
            }
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
        let mut lexer = Lexer::new(input.to_string());
        for t in expected_tokens {
            let next_token = lexer.next();

            match next_token {
                Some(tok) => {
                    assert_eq!(tok.token_type, t.token_type);
                    assert_eq!(tok.literal, t.literal);
                },
                None => { panic!("expected {:?}, got None", t) }
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
            Token::new(TokenType::NEWLINE,    "\n".to_string()),
            Token::new(TokenType::IDENT,      "ten".to_string()),
            Token::new(TokenType::ASSIGN,     "=".to_string()),
            Token::new(TokenType::INT,        "10".to_string()),
            Token::new(TokenType::NEWLINE,    "\n".to_string()),
            Token::new(TokenType::DEF,        "def".to_string()),
            Token::new(TokenType::IDENT,      "add".to_string()),
            Token::new(TokenType::LPAREN,     "(".to_string()),
            Token::new(TokenType::IDENT,      "x".to_string()),
            Token::new(TokenType::COMMA,      ",".to_string()),
            Token::new(TokenType::IDENT,      "y".to_string()),
            Token::new(TokenType::RPAREN,     ")".to_string()),
            Token::new(TokenType::NEWLINE,    "\n".to_string()),
            Token::new(TokenType::IDENT,      "x".to_string()),
            Token::new(TokenType::PLUS,       "+".to_string()),
            Token::new(TokenType::IDENT,      "y".to_string()),
            Token::new(TokenType::SEMICOLON,  ";".to_string()),
            Token::new(TokenType::NEWLINE,    "\n".to_string()),
            Token::new(TokenType::END,        "end".to_string()),
            Token::new(TokenType::NEWLINE,    "\n".to_string()),
            Token::new(TokenType::IDENT,      "result".to_string()),
            Token::new(TokenType::ASSIGN,     "=".to_string()),
            Token::new(TokenType::IDENT,      "add".to_string()),
            Token::new(TokenType::IDENT,      "five".to_string()),
            Token::new(TokenType::COMMA,      ",".to_string()),
            Token::new(TokenType::IDENT,      "ten".to_string()),
        ];

        let mut lexer = Lexer::new(input.to_string());
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
    fn it_can_parse_more_characters() {
        let input = "five = 5
ten = 10
def add(x, y)
  x + y
end
result = add(five, ten); !-/*5;
5 < 10 > 5;

if (5 < 10) {
  return true
} else {
  return false
}

10 == 10
10 != 9";

        let expected_tokens = vec![
            Token::new(TokenType::IDENT,      "five".to_string()),
            Token::new(TokenType::ASSIGN,     "=".to_string()),
            Token::new(TokenType::INT,        "5".to_string()),
            Token::new(TokenType::NEWLINE,    "\n".to_string()),
            Token::new(TokenType::IDENT,      "ten".to_string()),
            Token::new(TokenType::ASSIGN,     "=".to_string()),
            Token::new(TokenType::INT,        "10".to_string()),
            Token::new(TokenType::NEWLINE,    "\n".to_string()),
            Token::new(TokenType::DEF,        "def".to_string()),
            Token::new(TokenType::IDENT,      "add".to_string()),
            Token::new(TokenType::LPAREN,     "(".to_string()),
            Token::new(TokenType::IDENT,      "x".to_string()),
            Token::new(TokenType::COMMA,      ",".to_string()),
            Token::new(TokenType::IDENT,      "y".to_string()),
            Token::new(TokenType::RPAREN,     ")".to_string()),
            Token::new(TokenType::NEWLINE,    "\n".to_string()),
            Token::new(TokenType::IDENT,      "x".to_string()),
            Token::new(TokenType::PLUS,       "+".to_string()),
            Token::new(TokenType::IDENT,      "y".to_string()),
            Token::new(TokenType::NEWLINE,    "\n".to_string()),
            Token::new(TokenType::END,        "end".to_string()),
            Token::new(TokenType::NEWLINE,    "\n".to_string()),
            Token::new(TokenType::IDENT,      "result".to_string()),
            Token::new(TokenType::ASSIGN,     "=".to_string()),
            Token::new(TokenType::IDENT,      "add".to_string()),
            Token::new(TokenType::LPAREN,     "(".to_string()),
            Token::new(TokenType::IDENT,      "five".to_string()),
            Token::new(TokenType::COMMA,      ",".to_string()),
            Token::new(TokenType::IDENT,      "ten".to_string()),
            Token::new(TokenType::RPAREN,     ")".to_string()),
            Token::new(TokenType::SEMICOLON,  ";".to_string()),
            Token::new(TokenType::BANG,       "!".to_string()),
            Token::new(TokenType::MINUS,      "-".to_string()),
            Token::new(TokenType::SLASH,      "/".to_string()),
            Token::new(TokenType::ASTERISK,   "*".to_string()),
            Token::new(TokenType::INT,        "5".to_string()),
            Token::new(TokenType::SEMICOLON,  ";".to_string()),
            Token::new(TokenType::NEWLINE,    "\n".to_string()),
            Token::new(TokenType::INT,        "5".to_string()),
            Token::new(TokenType::LT,         "<".to_string()),
            Token::new(TokenType::INT,        "10".to_string()),
            Token::new(TokenType::GT,         ">".to_string()),
            Token::new(TokenType::INT,        "5".to_string()),
            Token::new(TokenType::SEMICOLON,  ";".to_string()),
            Token::new(TokenType::NEWLINE,    "\n".to_string()),
            Token::new(TokenType::NEWLINE,    "\n".to_string()),
            Token::new(TokenType::IF,         "if".to_string()),
            Token::new(TokenType::LPAREN,     "(".to_string()),
            Token::new(TokenType::INT,        "5".to_string()),
            Token::new(TokenType::LT,         "<".to_string()),
            Token::new(TokenType::INT,        "10".to_string()),
            Token::new(TokenType::RPAREN,     ")".to_string()),
            Token::new(TokenType::LBRACE,     "{".to_string()),
            Token::new(TokenType::NEWLINE,    "\n".to_string()),
            Token::new(TokenType::RETURN,     "return".to_string()),
            Token::new(TokenType::TRUE,       "true".to_string()),
            Token::new(TokenType::NEWLINE,    "\n".to_string()),
            Token::new(TokenType::RBRACE,     "}".to_string()),
            Token::new(TokenType::ELSE,       "else".to_string()),
            Token::new(TokenType::LBRACE,     "{".to_string()),
            Token::new(TokenType::NEWLINE,    "\n".to_string()),
            Token::new(TokenType::RETURN,     "return".to_string()),
            Token::new(TokenType::FALSE,      "false".to_string()),
            Token::new(TokenType::NEWLINE,    "\n".to_string()),
            Token::new(TokenType::RBRACE,     "}".to_string()),
            Token::new(TokenType::NEWLINE,    "\n".to_string()),
            Token::new(TokenType::NEWLINE,    "\n".to_string()),
            Token::new(TokenType::INT,        "10".to_string()),
            Token::new(TokenType::EQ,         "==".to_string()),
            Token::new(TokenType::INT,        "10".to_string()),
            Token::new(TokenType::NEWLINE,    "\n".to_string()),
            Token::new(TokenType::INT,        "10".to_string()),
            Token::new(TokenType::NOTEQ,      "!=".to_string()),
            Token::new(TokenType::INT,        "9".to_string()),
            Token::new(TokenType::EOF,        "\u{0}".to_string()),
        ];

        let mut lexer = Lexer::new(input.to_string());
        for t in expected_tokens {
            let next_token = lexer.next();

            match next_token {
                Some(tok) => {
                    println!("Compare {} and {}", tok.literal, t.literal);
                    assert_eq!(tok.token_type, t.token_type);
                    assert_eq!(tok.literal, t.literal);
                },
                None => { panic!("expected {:?}, got None", t) }
            }
        }
    }
}
