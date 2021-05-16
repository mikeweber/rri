use crate::lexer::token::Token;

#[derive(Clone, Debug)]
pub enum Node<'a> {
    Identifier(Token, String),
    NumberNode(Token),
    BinOpNode(&'a Node<'a>, &'a Node<'a>, Token)
}

pub fn token_literal(node: &Node) -> String {
    match node {
        Node::Identifier(token, _) => token.literal.clone(),
        Node::NumberNode(token) => token.literal.clone(),
        Node::BinOpNode(_, _, token) => token.literal.clone(),
    }
}
