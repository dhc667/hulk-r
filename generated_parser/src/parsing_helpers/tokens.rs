use ast::{
    BinaryOperator, BooleanLiteral, Identifier, NumberLiteral, StringLiteral, TokenPosition,
    TypeName, UnaryOperator,
};
use parser_generator::Token;

use crate::types::{ReturnType, TokenType};

pub(crate) fn identifier_from_default_token(ident: ReturnType) -> Identifier {
    let ident = ident.try_into_default_token().unwrap();

    let pos = get_pos(&ident);

    Identifier::new(pos.start, pos.end, &ident.slice)
}

pub(crate) fn type_name_from_default_token(ident: ReturnType) -> TypeName {
    let ident = ident.try_into_default_token().unwrap();

    let pos = get_pos(&ident);

    TypeName::new(pos.start, pos.end, ident.slice)
}

pub(crate) fn plus_minus_binary(mut v: Vec<ReturnType>) -> ReturnType {
    let tok = v.pop().unwrap().try_into_default_token().unwrap();

    let pos = get_pos(&tok);
    match tok.ty {
        TokenType::Plus => ReturnType::BinaryOperator(BinaryOperator::Plus(pos)),
        TokenType::Minus => ReturnType::BinaryOperator(BinaryOperator::Minus(pos)),
        _ => panic!("Expected Plus or Minus token for binary operator casting"),
    }
}

pub(crate) fn plus_minus_unary(mut v: Vec<ReturnType>) -> ReturnType {
    let tok = v.pop().unwrap().try_into_default_token().unwrap();
    let pos = get_pos(&tok);

    match tok.ty {
        TokenType::Plus => ReturnType::UnaryOperator(UnaryOperator::Plus(pos)),
        TokenType::Minus => ReturnType::UnaryOperator(UnaryOperator::Minus(pos)),
        _ => panic!("Expected Plus or Minus token for binary operator casting"),
    }
}

pub(crate) fn tok_to_string_literal(tok: &Token<TokenType>) -> ReturnType {
    let pos = get_pos(tok);
    ReturnType::StringLiteral(StringLiteral::new(
        pos.start,
        pos.end,
        &tok.slice
    ))
}

pub(crate) fn tok_to_boolean_literal(tok: &Token<TokenType>) -> ReturnType {
    let pos = get_pos(tok);
    let literal = match &tok.slice[..] {
        "true" => true,
        "false" => false,
        _ => panic!("Unrecognized boolean literal \"{}\"", &tok.slice),
    };
    let literal = BooleanLiteral::new(pos.start, pos.end, literal);

    ReturnType::BooleanLiteral(literal)
}

pub(crate) fn tok_to_number_literal(tok: &Token<TokenType>) -> ReturnType {
    let pos = get_pos(tok);
    let literal = NumberLiteral::new(pos.start, pos.end, &tok.slice);

    ReturnType::NumberLiteral(literal)
}

pub(crate) fn get_pos(tok: &Token<TokenType>) -> TokenPosition {
    TokenPosition::new(tok.start, tok.end)
}
