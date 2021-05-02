mod lexer;

#[cfg(test)]
mod tests {
    use crate::lexer::token::{ TokenType, Token };

    #[test]
    fn it_can_build_a_token() {
        let token = Token {
            token_type: TokenType::ILLEGAL,
            literal:    "?".to_string()
        };

        assert_eq!(token.token_type, TokenType::ILLEGAL);
        assert_eq!(token.literal, "?".to_string());
    }
}
