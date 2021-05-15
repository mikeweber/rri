use crate::lexer::token::Token;
use super::node::Node;

#[derive(Clone, Debug)]
pub enum Expression<'a> {
    Assign(Token, Node<'a>, Box<Expression<'a>>),
    Value(Token, i32),
    Return(Token, Box<Expression<'a>>),
}

impl<'a> Expression<'a> {
    pub fn token_literal(&self) -> String {
        match self {
            Expression::Assign(token, _, _) => token.literal.clone(),
            Expression::Value(token, _) => token.literal.clone(),
            Expression::Return(token, _) => token.literal.clone(),
        }
    }
}

