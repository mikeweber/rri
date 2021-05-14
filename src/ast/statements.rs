use crate::lexer::token::Token;
use crate::ast::node::Identifier;
use crate::ast::expressions::Expression;

pub enum Statement {
    Assign(Token, Identifier, Expression)
}
