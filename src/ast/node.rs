use crate::lexer::token::Token;

pub enum Node<'a> {
    Identifier(&'a Token, String),
    NumberNode(&'a Token),
    BinOpNode(&'a Node<'a>, &'a Node<'a>, &'a Token)
}

pub fn token_literal(node: &Node) -> String {
    match node {
        Node::Identifier(token, _) => token.literal.clone(),
        Node::NumberNode(token) => token.literal.clone(),
        Node::BinOpNode(_, _, token) => token.literal.clone(),
    }
}
