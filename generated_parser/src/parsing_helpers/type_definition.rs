use ast::{Identifier, InheritanceIndicator, Keyword, TypeDef, TypeName};
use parser_generator::Token;

use crate::{
    parsing_helpers::tokens::get_pos,
    types::{ReturnType, TokenType, TypeMemberDefinition},
};

pub fn to_type_definition(
    type_token: Keyword,
    type_name: Token<TokenType>,
    parameters: Vec<Identifier>,
    inheritance_indicator: Option<InheritanceIndicator>,
    members: Vec<TypeMemberDefinition>,
) -> ReturnType {
    let pos = get_pos(&type_name);

    let mut data_member_defs = Vec::new();
    let mut function_member_defs = Vec::new();

    members.into_iter().for_each(|member| match member {
        TypeMemberDefinition::FunctionMemberDef(function_def) => {
            function_member_defs.push(function_def)
        }
        TypeMemberDefinition::DataMemberDef(data_member_def) => {
            data_member_defs.push(data_member_def)
        }
    });

    ReturnType::Definition(ast::Definition::TypeDef(TypeDef::new(
        type_token,
        TypeName::new(pos.start, pos.end, type_name.slice),
        parameters,
        inheritance_indicator,
        data_member_defs,
        function_member_defs,
    )))
}
