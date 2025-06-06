use ast::{
    Assignment, BinOp, Block, BooleanLiteral, ConstantDef, DataMemberAccess, Definition,
    DefinitionVisitor, DestructiveAssignment, Expression, ExpressionVisitor, For, FunctionCall,
    FunctionMemberAccess, GlobalFunctionDef, Identifier, IfElse, LetIn, ListIndexing, ListLiteral,
    NewExpr, NumberLiteral, ProtocolDef, ReturnStatement, StringLiteral, TypeDef, UnOp,
    VisitableDefinition, VisitableExpression, While, typing::Type,
};
use generator::context::Context;

use crate::{def_info::TypeInfo, typing::get_conformable::GetConformable};

pub struct AnnotationVisitor<'a> {
    pub type_definitions: &'a mut Context<TypeInfo>,
    pub errors: &'a mut Vec<String>,
}

impl<'a> AnnotationVisitor<'a> {
    pub fn new(type_definitions: &'a mut Context<TypeInfo>, errors: &'a mut Vec<String>) -> Self {
        Self {
            type_definitions,
            errors,
        }
    }

    fn fix_annotation(&mut self, id: &mut Identifier) {
        match self.get_conformable(&id.info.ty) {
            Ok(_) => {}
            Err(message) => {
                self.errors.push(message);
                id.info.ty = None
            }
        }
    }
}

impl<'a> GetConformable for AnnotationVisitor<'a> {
    fn is_type_defined(&self, ty: &Type) -> bool {
        self.type_definitions.is_defined(&ty.to_string())
    }
}

impl<'a> DefinitionVisitor<()> for AnnotationVisitor<'a> {
    fn visit_definition(&mut self, node: &mut Definition) -> () {
        node.accept(self)
    }

    fn visit_type_def(&mut self, node: &mut TypeDef) -> () {
        // check type arguments
        for arg in &mut node.parameter_list {
            self.fix_annotation(arg);
        }

        // check fields declarations
        for member in &mut node.data_member_defs {
            self.fix_annotation(&mut member.identifier);
            member.default_value.accept(self);
        }

        // check functions declarations
        for func in &mut node.function_member_defs {
            // check return type
            self.fix_annotation(&mut func.identifier);
            // check return parameters
            for arg in &mut func.parameters {
                self.fix_annotation(arg);
            }
            // check body
            func.body.accept(self);
        }
    }

    fn visit_function_def(&mut self, node: &mut GlobalFunctionDef) -> () {
        // check return type
        self.fix_annotation(&mut node.function_def.identifier);
        // check return parameters
        for arg in &mut node.function_def.parameters {
            self.fix_annotation(arg);
        }
        // check body
        node.function_def.body.accept(self);
    }

    fn visit_constant_def(&mut self, node: &mut ConstantDef) -> () {
        self.fix_annotation(&mut node.identifier);
        node.initializer_expression.accept(self);
    }

    fn visit_protocol_def(&mut self, _node: &mut ProtocolDef) -> () {
        todo!()
    }
}

impl<'a> ExpressionVisitor<()> for AnnotationVisitor<'a> {
    fn visit_expression(&mut self, node: &mut Expression) -> () {
        node.accept(self);
    }

    fn visit_destructive_assignment(&mut self, node: &mut DestructiveAssignment) -> () {
        node.lhs.accept(self);
        node.rhs.accept(self);
    }

    fn visit_bin_op(&mut self, node: &mut BinOp) -> () {
        node.lhs.accept(self);
        node.rhs.accept(self);
    }

    fn visit_let_in(&mut self, node: &mut LetIn) -> () {
        node.assignment.accept(self);
        node.body.accept(self);
    }

    fn visit_new_expr(&mut self, node: &mut NewExpr) -> () {
        for arg in &mut node.arguments {
            arg.accept(self);
        }
    }

    fn visit_assignment(&mut self, node: &mut Assignment) -> () {
        self.fix_annotation(&mut node.identifier);
        node.rhs.accept(self);
    }

    fn visit_if_else(&mut self, node: &mut IfElse) -> () {
        node.condition.accept(self);
        node.then_expression.accept(self);
        node.else_expression.accept(self);
    }

    fn visit_while(&mut self, node: &mut While) -> () {
        node.condition.accept(self);
        node.body.accept(self);
    }

    fn visit_for(&mut self, node: &mut For) -> () {
        self.fix_annotation(&mut node.element);
        node.iterable.accept(self);
        node.body.accept(self);
    }

    fn visit_block(&mut self, node: &mut Block) -> () {
        for item in &mut node.body_items {
            item.accept(self);
        }
    }

    fn visit_return_statement(&mut self, node: &mut ReturnStatement) -> () {
        node.expression.accept(self);
    }

    fn visit_un_op(&mut self, node: &mut UnOp) -> () {
        node.rhs.accept(self);
    }

    fn visit_data_member_access(&mut self, node: &mut DataMemberAccess) -> () {
        node.object.accept(self);
    }

    fn visit_function_member_access(&mut self, node: &mut FunctionMemberAccess) -> () {
        node.object.accept(self);
    }

    fn visist_list_indexing(&mut self, node: &mut ListIndexing) -> () {
        node.list.accept(self);
        node.index.accept(self);
    }

    fn visit_function_call(&mut self, node: &mut FunctionCall) -> () {
        for arg in &mut node.arguments {
            arg.accept(self);
        }
    }

    fn visit_variable(&mut self, node: &mut Identifier) -> () {
        self.fix_annotation(node);
    }

    fn visit_number_literal(&mut self, _node: &mut NumberLiteral) -> () {}

    fn visit_boolean_literal(&mut self, _node: &mut BooleanLiteral) -> () {}

    fn visit_string_literal(&mut self, _node: &mut StringLiteral) -> () {}

    fn visit_list_literal(&mut self, node: &mut ListLiteral) -> () {
        for elem in &mut node.elements {
            elem.accept(self);
        }
    }

    fn visit_empty_expression(&mut self) -> () {}
}
