use crate::lexer::token::Token;
use crate::ast::node::Identifier;

pub enum Expression {
    Assign(&Token, Identifier, &Expression)
}
