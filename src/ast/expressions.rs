use crate::lexer::token::Token;
use super::node::Node;

#[derive(Clone, Debug)]
pub enum Expression<'a> {
    Assign(Token, Node<'a>, Box<Expression<'a>>),
    Value(Token, i32),
    Return(Token, Box<Expression<'a>>),
    Identifier(Token, Node<'a>),
}

impl<'a> Expression<'a> {
    pub fn token_literal(&self) -> String {
        match self {
            Expression::Assign(token, _, _) => token.literal.clone(),
            Expression::Value(token, _) => token.literal.clone(),
            Expression::Return(token, _) => token.literal.clone(),
            Expression::Identifier(token, _) => token.literal.clone(),
        }
    }

    pub fn to_s(&self) -> String {
        match self {
            Expression::Assign(token, ident, expr) => {
                match ident {
                    Node::Identifier(_, name) => format!("{} {} {}\n", name, token.literal, expr.to_s()),
                    _ => String::from("[INVALID ASSIGN EXPRESSION]")
                }
            },
            Expression::Value(token, value) => format!("{}", token.literal),
            Expression::Return(token, expr) => format!("{} {}\n", token.literal, expr.to_s()),
            Expression::Identifier(token, _) => format!("{}", token.literal)
        }
    }
}

