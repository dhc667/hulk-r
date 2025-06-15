use ast::{BooleanLiteral, Identifier, Keyword, Literal, NumberLiteral, Operator, StringLiteral, TokenPosition, TypeName};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct IdentifierToken {
    pub id: String,
    pub position: TokenPosition,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    Identifier(IdentifierToken),
    Keyword(Keyword),
    Literal(Literal),
    Operator(Operator),
    TypeName(TypeName),
}
