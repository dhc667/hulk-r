use ast::{
    typing::Type, ArrowExpression, ArrowOperator, Assignment, BinaryOperator, Block, BlockBodyItem, BooleanLiteral, DataMemberDef, Definition, DestructiveAssignment, DotOperator, Expression, FunctionCall, FunctionDef, FunctionSignature, GroupingOperator, Identifier, InheritanceIndicator, Keyword, ListLiteral, NumberLiteral, Program, StringLiteral, TypeName, UnaryOperator
};
use parser_generator::Token;

use crate::types::TokenType;

#[derive(Debug)]
pub enum ReturnType {
    Program(Program),
    InstructionList(Vec<Instruction>),
    Instruction(Instruction),

    Definition(Definition),
    FunctionSignature(FunctionSignature),

    TypeMemberDefinitionList(Vec<TypeMemberDefinition>),
    TypeMemberDefinition(TypeMemberDefinition),
    OptionalInheritanceIndicator(Option<InheritanceIndicator>),

    DestructiveAssignment(DestructiveAssignment),
    Expression(Expression),
    Block(Block),
    ArrowExpression(ArrowExpression),
    BlockBody(BlockBody),
    BlockBodyItem(BlockBodyItemReturn),
    /// true if more than one semicolon was parsed, false if only one was parsed
    MultipleSemicolons(bool),
    FunctionCall(FunctionCall),

    Keyword(Keyword),
    DefaultToken(Token<TokenType>),
    Type(Type),
    OptionalTypeAnnotation(Option<Type>),
    IdentifierNT(Identifier),

    NumberLiteral(NumberLiteral),
    BooleanLiteral(BooleanLiteral),
    StringLiteral(StringLiteral),
    ListLiteral(ListLiteral),

    GroupingOperator(GroupingOperator),
    BinaryOperator(BinaryOperator),
    UnaryOperator(UnaryOperator),
    ArrowOperator(ArrowOperator),
    DotOperator(DotOperator),

    AssignmentList(Vec<Assignment>),
    Assignment(Assignment),

    ArgumentList(Vec<Expression>),

    ParameterList(Vec<Identifier>),
}

impl ReturnType {
    pub fn try_into_default_token(self) -> Result<Token<TokenType>, Self> {
        if let Self::DefaultToken(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_keyword(self) -> Result<Keyword, Self> {
        if let Self::Keyword(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_number_literal(self) -> Result<NumberLiteral, Self> {
        if let Self::NumberLiteral(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_boolean_literal(self) -> Result<BooleanLiteral, Self> {
        if let Self::BooleanLiteral(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_string_literal(self) -> Result<StringLiteral, Self> {
        if let Self::StringLiteral(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_program(self) -> Result<Program, Self> {
        if let Self::Program(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_expression(self) -> Result<Expression, Self> {
        if let Self::Expression(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_definition(self) -> Result<Definition, Self> {
        if let Self::Definition(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_grouping_operator(self) -> Result<GroupingOperator, Self> {
        if let Self::GroupingOperator(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_binary_operator(self) -> Result<BinaryOperator, Self> {
        if let Self::BinaryOperator(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_arrow_operator(self) -> Result<ArrowOperator, Self> {
        if let Self::ArrowOperator(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_dot_operator(self) -> Result<DotOperator, Self> {
        if let Self::DotOperator(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_argument_list(self) -> Result<Vec<Expression>, Self> {
        if let Self::ArgumentList(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_destructive_assignment(self) -> Result<DestructiveAssignment, Self> {
        if let Self::DestructiveAssignment(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_unary_operator(self) -> Result<UnaryOperator, Self> {
        if let Self::UnaryOperator(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_assignment_list(self) -> Result<Vec<Assignment>, Self> {
        if let Self::AssignmentList(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_assignment(self) -> Result<Assignment, Self> {
        if let Self::Assignment(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_type(self) -> Result<Type, Self> {
        if let Self::Type(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_block(self) -> Result<Block, Self> {
        if let Self::Block(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_identifier_nt(self) -> Result<Identifier, Self> {
        if let Self::IdentifierNT(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_function_call(self) -> Result<FunctionCall, Self> {
        if let Self::FunctionCall(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_block_body(self) -> Result<BlockBody, Self> {
        if let Self::BlockBody(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_block_body_item(self) -> Result<BlockBodyItemReturn, Self> {
        if let Self::BlockBodyItem(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_multiple_semicolons(self) -> Result<bool, Self> {
        if let Self::MultipleSemicolons(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_list_literal(self) -> Result<ListLiteral, Self> {
        if let Self::ListLiteral(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_instruction_list(self) -> Result<Vec<Instruction>, Self> {
        if let Self::InstructionList(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_instruction(self) -> Result<Instruction, Self> {
        if let Self::Instruction(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_parameter_list(self) -> Result<Vec<Identifier>, Self> {
        if let Self::ParameterList(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_type_member_definition_list(self) -> Result<Vec<TypeMemberDefinition>, Self> {
        if let Self::TypeMemberDefinitionList(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_type_member_definition(self) -> Result<TypeMemberDefinition, Self> {
        if let Self::TypeMemberDefinition(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_optional_type_annotation(self) -> Result<Option<Type>, Self> {
        if let Self::OptionalTypeAnnotation(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_arrow_expression(self) -> Result<ArrowExpression, Self> {
        if let Self::ArrowExpression(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_function_signature(self) -> Result<FunctionSignature, Self> {
        if let Self::FunctionSignature(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_optional_inheritance_indicator(self) -> Result<Option<InheritanceIndicator>, Self> {
        if let Self::OptionalInheritanceIndicator(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

#[derive(Debug)]
pub struct BlockBody {
    pub items: Vec<BlockBodyItem>,
    pub multiple_semicolons: bool,
}

#[derive(Debug)]
pub struct BlockBodyItemReturn {
    pub item: BlockBodyItem,
    pub multiple_semicolons: bool,
}

#[derive(Debug)]
pub enum Instruction {
    Expression(Expression),
    Definition(Definition)
}

#[derive(Debug)]
pub enum TypeMemberDefinition {
    FunctionMemberDef(FunctionDef),
    DataMemberDef(DataMemberDef),
}

