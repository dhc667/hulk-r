use ast::{
    ArrowExpression, ArrowOperator, Assignment, Block, BlockBodyItem, ConstantDef,
    DataMemberAccess, DataMemberDef, Definition, DestructiveAssignment, DotOperator, Expression,
    For, FunctionBody, FunctionCall, FunctionDef, FunctionMemberAccess, GlobalFunctionDef,
    GroupingOperator, IfElse, InheritanceIndicator, Keyword, LetIn, ListIndexing, ListLiteral,
    NewExpr, ReturnStatement, UnOp, While,
    typing::{self, BuiltInType},
};
use parser_generator::{Parser, Token, grammar};

use crate::parsing_helpers;

#[allow(unused_imports)]
use crate::{
    get_last, get_pos,
    lexer_wrapper::{LexerDefiner, LexerWrapper},
    tok_to_boolean_literal, tok_to_number_literal, tok_to_string_literal,
    type_name_from_default_token,
    types::{
        BlockBody, BlockBodyItemReturn, Instruction, ReturnType, TokenType, TypeMemberDefinition,
    },
};

pub fn lexer_parser() -> (LexerWrapper, Parser<TokenType, ReturnType>) {
    grammar!(
        token_type: TokenType,
        return_type: ReturnType,
        lexer_definer_type: LexerDefiner,
        first_symbol: Program,
        default_token_action: |tok: &Token<TokenType>| {
            ReturnType::DefaultToken(tok.clone())
        },

        productions: {
            Program -> InstructionList
                = |mut v| {
                    let instructions = v.pop().unwrap().try_into_instruction_list().unwrap();

                    ReturnType::Program(parsing_helpers::program_from_instructions(instructions))
                }
            ;

            InstructionList -> InstructionList Instruction
                = |mut v| {
                    let last = v.pop().unwrap().try_into_instruction().unwrap();
                    let mut instructions = v.pop().unwrap().try_into_instruction_list().unwrap();
                    instructions.push(last);

                    ReturnType::InstructionList(instructions)
                }
            ;

            InstructionList -> Instruction
                = |mut v| {
                    let last = v.pop().unwrap().try_into_instruction().unwrap();
                    let mut instructions = Vec::new();
                    instructions.push(last);

                    ReturnType::InstructionList(instructions)
                }
            ;

            Instruction -> Definition
                = |mut v| {
                    let def = v.pop().unwrap().try_into_definition().unwrap();

                    ReturnType::Instruction(Instruction::Definition(def))
                }
            ;

            Instruction -> Expression Semicolon
                = |mut v| {
                    v.pop().unwrap();
                    let def = v.pop().unwrap().try_into_expression().unwrap();

                    ReturnType::Instruction(Instruction::Expression(def))
                }
            ;

            Definition -> TypeDef = get_last;
            // Definition -> ProtocolDef = get_last;
            Definition -> GlobalFunctionDef = get_last;
            Definition -> ConstantDef = get_last;

            TypeDef -> Type Identifier OptionalParameters OptionalInheritanceIndicator Lbrace OptionalTypeMembers Rbrace
                = |mut v| {
                    parsing_helpers::to_type_definition(
                        v.remove(0).try_into_keyword().unwrap(),
                        v.remove(0).try_into_default_token().unwrap(),
                        v.remove(0).try_into_parameter_list().unwrap(),
                        v.remove(0).try_into_optional_inheritance_indicator().unwrap(),
                        v.remove(1).try_into_type_member_definition_list().unwrap(),
                    )
                }
            ;

            OptionalParameters -> Parameters = get_last;
            OptionalParameters -> #Epsilon
                = |_| {
                    ReturnType::ParameterList(Vec::new())
                }
            ;

            Parameters -> Lpar ParameterList Rpar
                = |mut v| {
                    v.pop().unwrap();
                    v.pop().unwrap()
                }
            ;
            Parameters -> Lpar Rpar = |_| ReturnType::ParameterList(Vec::new());

            ParameterList -> ParameterList Comma Parameter
                = |mut v| {
                    let id = v.pop().unwrap().try_into_identifier_nt().unwrap();
                    v.pop().unwrap();
                    let mut l = v.pop().unwrap().try_into_parameter_list().unwrap();

                    l.push(id);

                    ReturnType::ParameterList(l)
                }
            ;
            ParameterList -> Parameter
                = |mut v| {
                    let id = v.pop().unwrap().try_into_identifier_nt().unwrap();
                    let mut l = Vec::new();

                    l.push(id);

                    ReturnType::ParameterList(l)
                }
            ;

            Parameter -> IdentifierNT TypeAnnotation
                = |mut v| {
                    let ty = v.pop().unwrap().try_into_type().unwrap();
                    let mut id = v.pop().unwrap().try_into_identifier_nt().unwrap();

                    id.annotate_type(ty);

                    ReturnType::IdentifierNT(id)
                }
            ;

            OptionalInheritanceIndicator -> Inherits Identifier OptionalArguments
                = |mut v| {
                    let args = v.pop().unwrap().try_into_argument_list().unwrap();
                    let ty_name = v.pop().unwrap();
                    let ty_name = parsing_helpers::type_name_from_default_token(ty_name);
                    let inherits_token = v.pop().unwrap().try_into_keyword().unwrap();

                    ReturnType::OptionalInheritanceIndicator(
                        Some(InheritanceIndicator::new(
                            inherits_token,
                            ty_name,
                            args
                        ))
                    )
                }
            ;
            OptionalInheritanceIndicator -> #Epsilon = |_| ReturnType::OptionalInheritanceIndicator(None);

            OptionalTypeMembers -> TypeMemberDefinitionList = get_last;
            OptionalTypeMembers -> #Epsilon = |_| ReturnType::TypeMemberDefinitionList(Vec::new());

            OptionalArguments -> Arguments = get_last;
            OptionalArguments -> #Epsilon
                = |_| ReturnType::ArgumentList(Vec::new())
            ;

            TypeMemberDefinitionList -> TypeMemberDefinitionList TypeMemberDefinition
                = |mut v| {
                    let m = v.pop().unwrap().try_into_type_member_definition().unwrap();
                    let mut l = v.pop().unwrap().try_into_type_member_definition_list().unwrap();

                    l.push(m);

                    ReturnType::TypeMemberDefinitionList(l)
                }
            ;
            TypeMemberDefinitionList -> TypeMemberDefinition
                = |mut v| {
                    let m = v.pop().unwrap().try_into_type_member_definition().unwrap();
                    let mut l = Vec::new();

                    l.push(m);

                    ReturnType::TypeMemberDefinitionList(l)
                }
            ;

            TypeMemberDefinition -> FunctionMemberDefinition = get_last;
            TypeMemberDefinition -> DataMemberDefinition = get_last;

            FunctionMemberDefinition -> IdentifierNT Parameters TypeAnnotation Block
                = |mut v| {
                    let b = v.pop().unwrap().try_into_block().unwrap();
                    let ty = v.pop().unwrap().try_into_type().unwrap();
                    let parameters = v.pop().unwrap().try_into_parameter_list().unwrap();
                    let mut id = v.pop().unwrap().try_into_identifier_nt().unwrap();

                    id.annotate_type(ty);

                    ReturnType::TypeMemberDefinition(
                        TypeMemberDefinition::FunctionMemberDef(
                            FunctionDef::new(
                                id,
                                parameters,
                                ast::FunctionBody::from(b)
                            )
                        )
                    )
                }
            ;
            FunctionMemberDefinition -> IdentifierNT Parameters OptionalTypeAnnotation ArrowExpression Semicolon
                = |mut v| {
                    v.pop().unwrap();
                    let a = v.pop().unwrap().try_into_arrow_expression().unwrap();
                    let ty = v.pop().unwrap().try_into_optional_type_annotation().unwrap();
                    let parameters = v.pop().unwrap().try_into_parameter_list().unwrap();
                    let mut id = v.pop().unwrap().try_into_identifier_nt().unwrap();

                    if let Some(ty) = ty {
                        id.annotate_type(ty);
                    }

                    ReturnType::TypeMemberDefinition(
                        TypeMemberDefinition::FunctionMemberDef(
                            FunctionDef::new(
                                id,
                                parameters,
                                FunctionBody::from(a)
                            )
                        )
                    )

                }
            ;

            OptionalTypeAnnotation -> TypeAnnotation
                = |mut v| ReturnType::OptionalTypeAnnotation(Some(v.pop().unwrap().try_into_type().unwrap()))
            ;
            OptionalTypeAnnotation -> #Epsilon
                = |mut v| ReturnType::OptionalTypeAnnotation(None)
            ;

            ArrowExpression -> Arrow Expression // 0.o
                = |mut v| {
                    let x = v.pop().unwrap().try_into_expression().unwrap();
                    let op = v.pop().unwrap().try_into_arrow_operator().unwrap();

                    ReturnType::ArrowExpression(ArrowExpression::new(op, x))
                }
            ;

            DataMemberDefinition -> Assignment Semicolon
                = |mut v| {
                    v.pop().unwrap();
                    let a = v.pop().unwrap().try_into_assignment().unwrap();

                    ReturnType::TypeMemberDefinition(
                        TypeMemberDefinition::DataMemberDef(
                            DataMemberDef::from(a)
                        )
                    )
                }
            ;

            GlobalFunctionDef -> Function IdentifierNT Parameters TypeAnnotation Block
                = |mut v| {
                    let b = v.pop().unwrap().try_into_block().unwrap();
                    let ty = v.pop().unwrap().try_into_type().unwrap();
                    let parameters = v.pop().unwrap().try_into_parameter_list().unwrap();
                    let mut id = v.pop().unwrap().try_into_identifier_nt().unwrap();
                    let function_token = v.pop().unwrap().try_into_keyword().unwrap();

                    id.annotate_type(ty);

                    ReturnType::Definition(
                        Definition::FunctionDef(
                            GlobalFunctionDef::new(
                                function_token,
                                id,
                                parameters,
                                FunctionBody::from(b)
                            )
                        )
                    )
                }
            ;
            GlobalFunctionDef -> Function IdentifierNT Parameters TypeAnnotation ArrowExpression Semicolon
                = |mut v| {
                    v.pop().unwrap();
                    let a = v.pop().unwrap().try_into_arrow_expression().unwrap();
                    let ty = v.pop().unwrap().try_into_type().unwrap();
                    let parameters = v.pop().unwrap().try_into_parameter_list().unwrap();
                    let mut id = v.pop().unwrap().try_into_identifier_nt().unwrap();
                    let function_token = v.pop().unwrap().try_into_keyword().unwrap();

                    id.annotate_type(ty);

                    ReturnType::Definition(
                        Definition::FunctionDef(
                            GlobalFunctionDef::new(
                                function_token,
                                id,
                                parameters,
                                FunctionBody::from(a)
                            )
                        )
                    )
                }
            ;

            ConstantDef -> Constant IdentifierNT TypeAnnotation Equal Expression Semicolon
                = |mut v| {
                    v.pop().unwrap();
                    let x = v.pop().unwrap().try_into_expression().unwrap();
                    let op = v.pop().unwrap().try_into_binary_operator().unwrap();
                    let ty = v.pop().unwrap().try_into_type().unwrap();
                    let mut id = v.pop().unwrap().try_into_identifier_nt().unwrap();
                    let constant_token = v.pop().unwrap().try_into_keyword().unwrap();

                    ReturnType::Definition(
                        Definition::ConstantDef(
                            ConstantDef::new(
                                constant_token,
                                ty,
                                id,
                                op,
                                x
                            )
                        )
                    )
                }
            ;

            Expression -> DestructiveAssignment
                = |mut v| ReturnType::Expression(
                    Expression::from(
                        v.pop()
                            .unwrap()
                            .try_into_destructive_assignment()
                            .unwrap()
                    )
                )
            ;

            Expression -> Concat
                = |mut v| ReturnType::Expression(
                    v.pop()
                        .unwrap()
                        .try_into_expression()
                        .unwrap()
                )
            ;

            DestructiveAssignment -> Atom ColonAssign Expression
                = |mut v| {
                    let r = v.pop().unwrap().try_into_expression().unwrap();
                    let op = v.pop().unwrap().try_into_binary_operator().unwrap();
                    let l = v.pop().unwrap().try_into_expression().unwrap();

                    ReturnType::DestructiveAssignment(
                        DestructiveAssignment::new(l, op, r)
                    )
                }
            ;

            Concat -> Concat ConcatOp LogicalOr = parsing_helpers::reduce_binop;
            Concat -> LogicalOr = parsing_helpers::get_last;

            LogicalOr -> LogicalOr Or LogicalAnd = parsing_helpers::reduce_binop;
            LogicalOr -> LogicalAnd = parsing_helpers::get_last;

            LogicalAnd -> LogicalAnd And Equation = parsing_helpers::reduce_binop;
            LogicalAnd -> Equation = parsing_helpers::get_last;

            Equation -> Comparison EqIneqOp Comparison = parsing_helpers::reduce_binop;
            Equation -> Comparison = parsing_helpers::get_last;

            Comparison -> Addition ComparisonOp Addition = parsing_helpers::reduce_binop;
            Comparison -> Addition = parsing_helpers::get_last;

            Addition -> Addition PlusMinusBinaryOp Term = parsing_helpers::reduce_binop;
            Addition -> Term = parsing_helpers::get_last;

            Term -> Term MultDivOp Factor = parsing_helpers::reduce_binop;
            Term -> Factor = parsing_helpers::get_last;

            Factor -> UnaryOperation = parsing_helpers::get_last;

            UnaryOperation -> PlusMinusNotUnaryOp UnaryOperation
                = |mut v| {
                    let rhs =  v.pop().unwrap().try_into_expression().unwrap();
                    let op = v.pop().unwrap().try_into_unary_operator().unwrap();

                    ReturnType::Expression(Expression::from(
                        UnOp::new(op, rhs)
                    ))
                }
            ;
            UnaryOperation -> CompositeExpression = get_last;

            CompositeExpression -> LetExpression = get_last;
            CompositeExpression -> IfExpression = get_last;
            CompositeExpression -> WhileExpression = get_last;
            // CompositeExpression -> ForExpression = get_last;
            CompositeExpression -> Atom = get_last;

            LetExpression -> Let AssignmentList In CompositeExpression
                = |mut v| {
                    let rhs = v.pop().unwrap().try_into_expression().unwrap();
                    let in_kwd = v.pop().unwrap().try_into_keyword().unwrap();
                    let assignment_list = v.pop().unwrap().try_into_assignment_list().unwrap();
                    let let_kwd = v.pop().unwrap().try_into_keyword().unwrap();

                    ReturnType::Expression(Expression::from(
                        LetIn::new(let_kwd, assignment_list, in_kwd, rhs)
                    ))
                }
            ;

            AssignmentList -> AssignmentList Comma Assignment
                = |mut v| {
                    let e = v.pop().unwrap().try_into_assignment().unwrap();
                    v.pop().unwrap();
                    let mut l = v.pop().unwrap().try_into_assignment_list().unwrap();

                    l.push(e);

                    ReturnType::AssignmentList(l)
                }
            ;
            AssignmentList -> Assignment
                = |mut v| {
                    let e = v.pop().unwrap().try_into_assignment().unwrap();
                    let mut l = Vec::new();
                    l.push(e);

                    ReturnType::AssignmentList(l)
                }
            ;

            Assignment -> Identifier TypeAnnotation Equal Expression
                = |mut v| {
                    let rhs = v.pop().unwrap().try_into_expression().unwrap();
                    let op = v.pop().unwrap().try_into_binary_operator().unwrap();
                    let ty = v.pop().unwrap().try_into_type().unwrap();
                    let mut id = parsing_helpers::identifier_from_default_token(v.pop().unwrap());

                    id.annotate_type(ty);

                    ReturnType::Assignment(Assignment::new(id, op, rhs))
                }
            ;
            Assignment -> Identifier Equal Expression
                = |mut v| {
                    let rhs = v.pop().unwrap().try_into_expression().unwrap();
                    let op = v.pop().unwrap().try_into_binary_operator().unwrap();
                    let mut id = parsing_helpers::identifier_from_default_token(v.pop().unwrap());

                    ReturnType::Assignment(Assignment::new(id, op, rhs))
                }
            ;

            TypeAnnotation -> Colon TypeNT = get_last;
            TypeNT -> TypeNT Times
                = |mut v| {
                    v.pop().unwrap();
                    let ty = v.pop().unwrap().try_into_type().unwrap();
                    ReturnType::Type(typing::Type::Iterable(Box::new(ty)))
                }
            ;
            // TypeNT -> Object = get_last;
            TypeNT -> Boolean = get_last;
            TypeNT -> String = get_last;
            TypeNT -> Number = get_last;
            TypeNT -> Identifier
                = |mut v| {
                    let id = v.pop().unwrap();
                    let ty = parsing_helpers::type_name_from_default_token(id);

                    ReturnType::Type(typing::Type::Defined(ty))
                }
            ;

            IfExpression -> If Lpar Expression Rpar CompositeExpression Else CompositeExpression
                = |mut v| {
                    let else_expression = v.pop().unwrap().try_into_expression().unwrap();
                    let else_token = v.pop().unwrap().try_into_keyword().unwrap();
                    let then_expression = v.pop().unwrap().try_into_expression().unwrap();
                    v.pop().unwrap();
                    let condition = v.pop().unwrap().try_into_expression().unwrap();
                    v.pop().unwrap();
                    let if_token = v.pop().unwrap().try_into_keyword().unwrap();


                    ReturnType::Expression(Expression::from(
                        IfElse::new(if_token, condition, then_expression, else_token, else_expression)
                    ))
                }
            ;

            WhileExpression -> While Lpar Expression Rpar Block
                = |mut v| {
                    let body = v.pop().unwrap().try_into_block().unwrap();
                    v.pop().unwrap();
                    let condition = v.pop().unwrap().try_into_expression().unwrap();
                    v.pop().unwrap();
                    let while_token = v.pop().unwrap().try_into_keyword().unwrap();


                    ReturnType::Expression(Expression::from(
                        While::new(while_token, condition, body)
                    ))
                }
            ;

            // ForExpression -> For Lpar IdentifierNT In Expression Rpar Block
            //     = |mut v| {
            //         let body = v.pop().unwrap().try_into_block().unwrap();
            //         v.pop().unwrap();
            //         let iterable = v.pop().unwrap().try_into_expression().unwrap();
            //         let in_token = v.pop().unwrap().try_into_keyword().unwrap();
            //         let element = v.pop().unwrap().try_into_identifier_nt().unwrap();
            //         v.pop().unwrap();
            //         let for_token = v.pop().unwrap().try_into_keyword().unwrap();


            //         ReturnType::Expression(Expression::from(
            //             For::new(for_token, element, in_token, iterable, body)
            //         ))
            //     }
            // ;

            Atom -> Lpar Expression Rpar
                = |mut v| {
                    v.pop().unwrap();
                    v.pop().unwrap()
                }
            ;
            Atom -> NewExpression = get_last;
            Atom -> FunctionCallExpression = get_last;
            Atom -> BlockExpression = get_last;

            Atom -> FunctionMemberAccess = get_last;
            Atom -> DataMemberAccess = get_last;
            Atom -> ListIndexingExpression = get_last;

            Atom -> ListLiteralExpression = get_last;
            Atom -> NumberLiteralExpression = get_last;
            Atom -> BooleanLiteralExpression = get_last;
            Atom -> StringLiteralExpression = get_last;
            Atom -> VariableExpression = get_last;

            NewExpression -> New FunctionCall
                = |mut v| {
                    let f = v.pop().unwrap().try_into_function_call().unwrap();
                    let k = v.pop().unwrap().try_into_keyword().unwrap();

                    ReturnType::Expression(Expression::from(
                        NewExpr::from_function_call(k, f)
                    ))
                }
            ;

            FunctionCallExpression -> FunctionCall
                = |mut v| {
                    let f = v.pop().unwrap().try_into_function_call().unwrap();

                    ReturnType::Expression(Expression::from(f))
                }
            ;

            FunctionCall -> IdentifierNT Arguments
                = |mut v| {
                    let args = v.pop().unwrap().try_into_argument_list().unwrap();
                    let id = v.pop().unwrap().try_into_identifier_nt().unwrap();

                    ReturnType::FunctionCall(
                        FunctionCall::new(id, args)
                    )
                }
            ;

            Arguments -> Lpar ArgumentList Rpar
                = |mut v| {
                    v.pop().unwrap();
                    let args = v.pop().unwrap().try_into_argument_list().unwrap();

                    ReturnType::ArgumentList(args)
                }
            ;

            Arguments -> Lpar Rpar
                = |mut v| {
                    let args = Vec::new();

                    ReturnType::ArgumentList(args)
                }
            ;

            ArgumentList -> ArgumentList Comma Expression
                = |mut v| {
                    let x = v.pop().unwrap().try_into_expression().unwrap();
                    v.pop().unwrap();
                    let mut l = v.pop().unwrap().try_into_argument_list().unwrap();

                    l.push(x);

                    ReturnType::ArgumentList(l)
                }
            ;
            ArgumentList -> Expression
                = |mut v| {
                    let x = v.pop().unwrap().try_into_expression().unwrap();
                    let mut l = Vec::new();

                    l.push(x);

                    ReturnType::ArgumentList(l)
                }
            ;
            BlockExpression -> Block
                = |mut v| {
                    let block = v.pop().unwrap().try_into_block().unwrap();

                    ReturnType::Expression(Expression::from(block))
                }
            ;
            FunctionMemberAccess -> Atom Dot FunctionCall
                = |mut v| {
                    let f = v.pop().unwrap().try_into_function_call().unwrap();
                    let op = v.pop().unwrap().try_into_dot_operator().unwrap();
                    let object = v.pop().unwrap().try_into_expression().unwrap();

                    ReturnType::Expression(Expression::from(
                        FunctionMemberAccess::new(object, op, f)
                    ))
                }
            ;

            DataMemberAccess -> Atom Dot IdentifierNT
                = |mut v| {
                    let member = v.pop().unwrap().try_into_identifier_nt().unwrap();
                    let op = v.pop().unwrap().try_into_dot_operator().unwrap();
                    let object = v.pop().unwrap().try_into_expression().unwrap();

                    ReturnType::Expression(Expression::from(
                        DataMemberAccess::new(object, op, member)
                    ))
                }
            ;

            ListIndexingExpression -> Atom Lbracket Expression Rbracket
                = |mut v| {
                    let rb = v.pop().unwrap().try_into_grouping_operator().unwrap();
                    let index = v.pop().unwrap().try_into_expression().unwrap();
                    let lb = v.pop().unwrap().try_into_grouping_operator().unwrap();
                    let list = v.pop().unwrap().try_into_expression().unwrap();

                    ReturnType::Expression(Expression::from(
                        ListIndexing::new(list, lb, rb, index)
                    ))
                }
            ;

            VariableExpression -> IdentifierNT
                = |mut v| {
                    let v = v.pop().unwrap().try_into_identifier_nt().unwrap();

                    ReturnType::Expression(Expression::from(v))
                }
            ;
            IdentifierNT -> Identifier
                = |mut v| {
                    let id = parsing_helpers::identifier_from_default_token(v.pop().unwrap());

                    ReturnType::IdentifierNT(id)
                }
            ;

            ListLiteralExpression -> ListLiteral
                = |mut v| {
                    let l = v.pop().unwrap().try_into_list_literal().unwrap();

                    ReturnType::Expression(Expression::ListLiteral(l))
                }
            ;
            ListLiteral -> Lbracket ArgumentList Rbracket
                = |mut v| {
                    let right_brack = v.pop().unwrap().try_into_grouping_operator().unwrap();
                    let exp_list = v.pop().unwrap().try_into_argument_list().unwrap();
                    let left_brack = v.pop().unwrap().try_into_grouping_operator().unwrap();

                    ReturnType::ListLiteral(ListLiteral::new(left_brack, right_brack, exp_list))
                }
            ;
            ListLiteral -> Lbracket Rbracket
                = |mut v| {
                    let right_brack = v.pop().unwrap().try_into_grouping_operator().unwrap();
                    let exp_list = Vec::new();
                    let left_brack = v.pop().unwrap().try_into_grouping_operator().unwrap();

                    ReturnType::ListLiteral(ListLiteral::new(left_brack, right_brack, exp_list))
                }
            ;

            NumberLiteralExpression -> NumberLiteral
                = |mut v| {
                    ReturnType::Expression(Expression::from(v.pop().unwrap().try_into_number_literal().unwrap()))
                }
            ;

            BooleanLiteralExpression -> BooleanLiteral
                = |mut v| {
                    ReturnType::Expression(Expression::from(v.pop().unwrap().try_into_boolean_literal().unwrap()))
                }
            ;

            StringLiteralExpression -> StringLiteral
                = |mut v| {
                    ReturnType::Expression(Expression::from(v.pop().unwrap().try_into_string_literal().unwrap()))
                }
            ;
            Block -> Lbrace BlockBody Rbrace
                = |mut v| {
                    let right_brace = v.pop().unwrap().try_into_grouping_operator().unwrap();
                    let BlockBody {items, multiple_semicolons} = v.pop().unwrap().try_into_block_body().unwrap();
                    let left_brace = v.pop().unwrap().try_into_grouping_operator().unwrap();

                    ReturnType::Block(Block::new(left_brace, right_brace, items, multiple_semicolons))
                }
            ;

            BlockBody -> BlockBody BlockBodyItem
                = |mut v| {
                    let BlockBodyItemReturn {item, multiple_semicolons} = v.pop().unwrap().try_into_block_body_item().unwrap();
                    let BlockBody {mut items, multiple_semicolons: _} = v.pop().unwrap().try_into_block_body().unwrap();
                    items.push(item);

                    ReturnType::BlockBody(BlockBody {
                        items,
                        multiple_semicolons
                    })
                }
            ;
            BlockBody -> BlockBodyItem
                = |mut v| {
                    let BlockBodyItemReturn {item, multiple_semicolons} = v.pop().unwrap().try_into_block_body_item().unwrap();
                    let mut items = Vec::new();
                    items.push(item);

                    ReturnType::BlockBody(BlockBody {
                        items,
                        multiple_semicolons
                    })
                }
            ;

            BlockBodyItem -> Expression MultipleSemicolons
                = |mut v| {
                    let multiple_semicolons = v.pop().unwrap().try_into_multiple_semicolons().unwrap();
                    let expression = v.pop().unwrap().try_into_expression().unwrap();

                    ReturnType::BlockBodyItem(BlockBodyItemReturn {
                        item: BlockBodyItem::Expression(
                            expression
                        ),
                        multiple_semicolons
                    })
                }
            ;
            BlockBodyItem -> Return Expression MultipleSemicolons
                = |mut v| {
                    let multiple_semicolons = v.pop().unwrap().try_into_multiple_semicolons().unwrap();
                    let expression = v.pop().unwrap().try_into_expression().unwrap();
                    let k = v.pop().unwrap().try_into_keyword().unwrap();

                    ReturnType::BlockBodyItem(BlockBodyItemReturn {
                        item: BlockBodyItem::ReturnStatement(
                            ReturnStatement::new(k, expression)
                        ),
                        multiple_semicolons
                    })
                }
            ;

            MultipleSemicolons -> MultipleSemicolons Semicolon
                = |mut v| ReturnType::MultipleSemicolons(true)
            ;
            MultipleSemicolons -> Semicolon
                = |mut v| ReturnType::MultipleSemicolons(false)
            ;

            ConcatOp -> At = |mut v| {
                v.pop().unwrap()
            };
            ConcatOp -> AtAt = |mut v| {
                v.pop().unwrap()
            };

            EqIneqOp -> EqualEqual = get_last;
            EqIneqOp -> NotEqual = get_last;

            ComparisonOp -> Less = get_last;
            ComparisonOp -> LessEqual = get_last;
            ComparisonOp -> Greater = get_last;
            ComparisonOp -> GreaterEqual = get_last;

            PlusMinusBinaryOp -> Plus = parsing_helpers::plus_minus_binary;
            PlusMinusBinaryOp -> Minus = parsing_helpers::plus_minus_binary;

            MultDivOp -> Times = get_last;
            MultDivOp -> Div = get_last;

            PlusMinusNotUnaryOp -> PlusMinusUnaryOp = get_last;
            PlusMinusNotUnaryOp -> Not = get_last;

            PlusMinusUnaryOp -> Plus = parsing_helpers::plus_minus_unary;
            PlusMinusUnaryOp -> Minus = parsing_helpers::plus_minus_unary;

        }

        terminals: {

            (Let, "let", |tok: &Token<TokenType>| {
                ReturnType::Keyword(Keyword::Let(get_pos(tok)))
            }),
            (If, "if", |tok: &Token<TokenType>| {
                ReturnType::Keyword(Keyword::If(get_pos(tok)))
            }),
            (Else, "else", |tok: &Token<TokenType>| {
                ReturnType::Keyword(Keyword::Else(get_pos(tok)))
            }),
            (While, "while", |tok: &Token<TokenType>| {
                ReturnType::Keyword(Keyword::While(get_pos(tok)))
            }),
            // (For, "for", |tok: &Token<TokenType>| {
            //     ReturnType::Keyword(Keyword::For(get_pos(tok)))
            // }),
            (In, "in", |tok: &Token<TokenType>| {
                ReturnType::Keyword(Keyword::In(get_pos(tok)))
            }),
            (Elif, "elif", |tok: &Token<TokenType>| {
                ReturnType::Keyword(Keyword::Elif(get_pos(tok)))
            }),
            (New, "new", |tok: &Token<TokenType>| {
                ReturnType::Keyword(Keyword::New(get_pos(tok)))
            }),

            (Function, "function", |tok: &Token<TokenType>| {
                ReturnType::Keyword(Keyword::Function(get_pos(tok)))
            }),
            (Type, "type", |tok: &Token<TokenType>| {
                ReturnType::Keyword(Keyword::Type(get_pos(tok)))
            }),
            (Inherits, "inherits", |tok: &Token<TokenType>| {
                ReturnType::Keyword(Keyword::Inherits(get_pos(tok)))
            }),
            (Constant, "constant", |tok: &Token<TokenType>| {
                ReturnType::Keyword(Keyword::Constant(get_pos(tok)))
            }),
            // (Protocol, "protocol", |tok: &Token<TokenType>| {
            //     ReturnType::Keyword(Keyword::Protocol(get_pos(tok)))
            // }),
            (Extends, "extends", |tok: &Token<TokenType>| {
                ReturnType::Keyword(Keyword::Extends(get_pos(tok)))
            }),
            (Return, "return", |tok: &Token<TokenType>| {
                ReturnType::Keyword(Keyword::Return(get_pos(tok)))
            }),

            // (Object, "Object", |_: &Token<TokenType>| {
            //     ReturnType::Type(typing::Type::BuiltIn(BuiltInType::Object))
            // }),
            (Number, "Number", |_: &Token<TokenType>| {
                ReturnType::Type(typing::Type::BuiltIn(BuiltInType::Number))
            }),
            (String, "String", |_: &Token<TokenType>| {
                ReturnType::Type(typing::Type::BuiltIn(BuiltInType::String))
            }),
            (Boolean, "Boolean", |_: &Token<TokenType>| {
                ReturnType::Type(typing::Type::BuiltIn(BuiltInType::Bool))
            }),

            (BooleanLiteral, "((true)|(false))", tok_to_boolean_literal),
            (NumberLiteral, r"[0-9]*(\.[0-9]+)?", tok_to_number_literal),
            (StringLiteral, r#""([^\\-\\"-"]|(\\"))*""#, tok_to_string_literal),

            (Lpar, r"\(", |tok: &Token<TokenType>| {
                ReturnType::GroupingOperator(GroupingOperator::OpenParen(get_pos(tok)))
            }),
            (Rpar, r"\)", |tok: &Token<TokenType>| {
                ReturnType::GroupingOperator(GroupingOperator::CloseParen(get_pos(tok)))
            }),
            (Lbrace, r"{", |tok: &Token<TokenType>| {
                ReturnType::GroupingOperator(GroupingOperator::OpenBrace(get_pos(tok)))
            }),
            (Rbrace, r"}", |tok: &Token<TokenType>| {
                ReturnType::GroupingOperator(GroupingOperator::CloseBrace(get_pos(tok)))
            }),
            (Lbracket, r"\[", |tok: &Token<TokenType>| {
                ReturnType::GroupingOperator(GroupingOperator::OpenBracket(get_pos(tok)))
            }),
            (Rbracket, r"\]", |tok: &Token<TokenType>| {
                ReturnType::GroupingOperator(GroupingOperator::CloseBracket(get_pos(tok)))
            }),

            (At, "@", |tok: &Token<TokenType>| {
                ReturnType::BinaryOperator(ast::BinaryOperator::At(get_pos(tok)))
            }),
            (AtAt, "@@", |tok: &Token<TokenType>| {
                ReturnType::BinaryOperator(ast::BinaryOperator::AtAt(get_pos(tok)))
            }),
            (ColonAssign, ":=", |tok: &Token<TokenType>| {
                ReturnType::BinaryOperator(ast::BinaryOperator::ColonEqual(get_pos(tok)))
            }),
            (Or, r"\|\|", |tok: &Token<TokenType>| {
                ReturnType::BinaryOperator(ast::BinaryOperator::Or(get_pos(tok)))
            }),
            (And, "&&", |tok: &Token<TokenType>| {
                ReturnType::BinaryOperator(ast::BinaryOperator::And(get_pos(tok)))
            }),
            (Not, "!", |tok: &Token<TokenType>| {
                ReturnType::UnaryOperator(ast::UnaryOperator::Not(get_pos(tok)))
            }),
            (Equal, "=", |tok: &Token<TokenType>| {
                ReturnType::BinaryOperator(ast::BinaryOperator::Equal(get_pos(tok)))
            }),
            (EqualEqual, "==", |tok: &Token<TokenType>| {
                ReturnType::BinaryOperator(ast::BinaryOperator::EqualEqual(get_pos(tok)))
            }),
            (NotEqual, "!=", |tok: &Token<TokenType>| {
                ReturnType::BinaryOperator(ast::BinaryOperator::NotEqual(get_pos(tok)))
            }),
            (Arrow, "=>", |tok: &Token<TokenType>| {
                ReturnType::ArrowOperator(ArrowOperator::new(get_pos(tok)))
            }),
            (Less, "<", |tok: &Token<TokenType>| {
                ReturnType::BinaryOperator(ast::BinaryOperator::Less(get_pos(tok)))
            }),
            (LessEqual, "<=", |tok: &Token<TokenType>| {
                ReturnType::BinaryOperator(ast::BinaryOperator::LessEqual(get_pos(tok)))
            }),
            (Greater, ">", |tok: &Token<TokenType>| {
                ReturnType::BinaryOperator(ast::BinaryOperator::Greater(get_pos(tok)))
            }),
            (GreaterEqual, ">=", |tok: &Token<TokenType>| {
                ReturnType::BinaryOperator(ast::BinaryOperator::GreaterEqual(get_pos(tok)))
            }),

            (Plus, r"\+"), // we don't know if it's unary or binary
            (Minus, r"\-"), // we don't know if it's unary or binary
            (Times, r"\*", |tok: &Token<TokenType>| {
                ReturnType::BinaryOperator(ast::BinaryOperator::Times(get_pos(tok)))
            }),
            (Div, "/", |tok: &Token<TokenType>| {
                ReturnType::BinaryOperator(ast::BinaryOperator::Divide(get_pos(tok)))
            }),

            (Colon, ":"),
            (Semicolon, ";"),
            (Comma, ","),
            (Dot, r"\.", |tok: &Token<TokenType>| {
                ReturnType::DotOperator(DotOperator::new(get_pos(tok)))
            }),

            (Identifier, "[A-Za-z][A-Za-z0-9_-_]*"),
        }

        skip: {
            (__Whitespace__, r"(\s|\t|\n)+")
        }

    )
}
