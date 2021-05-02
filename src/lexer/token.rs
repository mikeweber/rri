pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(tt: TokenType, lit: String) -> Self {
        Self {
            token_type: tt,
            literal:    lit
        }
    }
}

#[derive(Debug)]
#[derive(std::cmp::PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers and literals
    IDENT,
    INT,

    // Operators
    ASSIGN,
    PLUS,

    // Delimiters
    COMMA,
    SEMICOLON,

    // Groupings
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    DEF,
    END,
}
