use phf::phf_map;

#[derive(Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
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
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,

    // Delimiters
    COMMA,
    SEMICOLON,

    // Groupings
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    NEWLINE,

    // Keywords
    DEF,
    END,
    DO,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    EQ,
    NOTEQ,
}

pub static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "def"    => TokenType::DEF,
    "end"    => TokenType::END,
    "do"     => TokenType::DO,
    "true"   => TokenType::TRUE,
    "false"  => TokenType::FALSE,
    "if"     => TokenType::IF,
    "else"   => TokenType::ELSE,
    "return" => TokenType::RETURN,
};
