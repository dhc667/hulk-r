use ast::{
    Assignment, BinOp, Block, BooleanLiteral, ConstantDef, DataMemberAccess, Definition,
    DefinitionVisitor, DestructiveAssignment, Expression, ExpressionVisitor, For, FunctionCall,
    FunctionMemberAccess, GlobalFunctionDef, Identifier, IfElse, LetIn, ListIndexing, ListLiteral,
    NewExpr, NumberLiteral, ProtocolDef, ReturnStatement, StringLiteral, TypeDef, UnOp,
    VisitableDefinition, VisitableExpression, While,
    typing::{Type, TypeAnnotation},
};
use error_handler::error::error::HulkError;
use generator::context::Context;

use crate::{
    def_info::{DefinedTypeInfo, FuncInfo, TypeInfo, VarInfo},
    typing::get_conformable::GetConformable,
};

pub struct AnnotationVisitor<'a> {
    pub type_definitions: &'a mut Context<TypeInfo>,
    pub func_definitions: &'a mut Context<FuncInfo>,
    pub errors: &'a mut Vec<HulkError>,
}

impl<'a> AnnotationVisitor<'a> {
    pub fn new(
        type_definitions: &'a mut Context<TypeInfo>,
        func_definitions: &'a mut Context<FuncInfo>,
        errors: &'a mut Vec<HulkError>,
    ) -> Self {
        Self {
            type_definitions,
            func_definitions,
            errors,
        }
    }

    fn fix_annotation(&mut self, id: &mut Identifier) {
        match self.get_conformable(&id.info.ty, id.position.start) {
            Ok(_) => {}
            Err(message) => {
                self.errors.push(message);
                id.info.ty = None
            }
        }
    }

    fn get_type_unsafe_mut(&mut self, id: &str) -> &mut DefinedTypeInfo {
        self.type_definitions
            .get_value_mut(&id)
            .expect("Type definition not found")
            .as_defined_mut()
            .expect("Type definition should be defined")
    }

    fn get_member_unsafe_mut(&mut self, type_id: &str, member_id: &str) -> &mut VarInfo {
        self.get_type_unsafe_mut(type_id)
            .members
            .get_mut(member_id)
            .expect("Member not found in type definition")
            .as_var_mut()
            .expect("Member should be a variable")
    }

    fn get_method_unsafe_mut(&mut self, type_id: &str, func_id: &str) -> &mut FuncInfo {
        self.get_type_unsafe_mut(type_id)
            .members
            .get_mut(func_id)
            .expect("Function not found in type definition")
            .as_func_mut()
            .expect("Function should be a variable")
    }

    fn get_func_unsafe_mut(&mut self, func_id: &str) -> &mut FuncInfo {
        self.func_definitions
            .get_value_mut(&func_id)
            .expect("Function definition not found")
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
        let mut args_to_fix = vec![];
        for (i, arg) in node.parameter_list.iter_mut().enumerate() {
            match self.get_conformable(&arg.info.ty, arg.position.start) {
                Ok(_) => {}
                Err(message) => {
                    self.errors.push(message);
                    arg.info.ty = None;
                    args_to_fix.push(i);
                }
            }
        }
        let args: &mut Vec<TypeAnnotation> = self
            .get_type_unsafe_mut(&node.name.id)
            .arguments_types
            .as_mut();

        for i in args_to_fix {
            args[i] = None;
        }

        // check fields declarations
        let mut members_to_fix = vec![];
        for member in node.data_member_defs.iter_mut() {
            match self.get_conformable(&member.identifier.info.ty, member.identifier.position.start)
            {
                Ok(_) => {}
                Err(message) => {
                    self.errors.push(message);
                    member.identifier.info.ty = None;
                    members_to_fix.push(member.identifier.id.clone());
                }
            }
            member.default_value.accept(self);
        }

        for member in members_to_fix {
            // might be less efficient inlining this, but it is more readable, and technically is O(1) ;)
            let member_info = self.get_member_unsafe_mut(&node.name.id, &member);
            member_info.ty = None;
        }

        // check functions declarations
        for func in &mut node.function_member_defs {
            // check return type
            let mut fix_return_type = false;
            match self.get_conformable(&func.identifier.info.ty, func.identifier.position.start) {
                Ok(_) => {}
                Err(message) => {
                    self.errors.push(message);
                    func.identifier.info.ty = None;
                    fix_return_type = true;
                }
            }

            // check arg types
            let mut args_to_fix = vec![];
            for (i, arg) in &mut func.parameters.iter_mut().enumerate() {
                match self.get_conformable(&arg.info.ty, arg.position.start) {
                    Ok(_) => {}
                    Err(message) => {
                        self.errors.push(message);
                        arg.info.ty = None;
                        args_to_fix.push(i);
                    }
                }
            }
            let func_info = self.get_method_unsafe_mut(&node.name.id, &func.identifier.id);
            for i in args_to_fix {
                func_info.parameters[i].info.ty = None;
            }
            if fix_return_type {
                func_info.name.info.ty = None;
            }
            // check body
            func.body.accept(self);
        }
    }

    fn visit_function_def(&mut self, node: &mut GlobalFunctionDef) -> () {
        let func = &mut node.function_def;
        // check return type
        let mut fix_return_type = false;
        match self.get_conformable(&func.identifier.info.ty, func.identifier.position.start) {
            Ok(_) => {}
            Err(message) => {
                self.errors.push(message);
                func.identifier.info.ty = None;
                fix_return_type = true;
            }
        }

        // check arg types
        let mut args_to_fix = vec![];
        for (i, arg) in &mut func.parameters.iter_mut().enumerate() {
            match self.get_conformable(&arg.info.ty, arg.position.start) {
                Ok(_) => {}
                Err(message) => {
                    self.errors.push(message);
                    arg.info.ty = None;
                    args_to_fix.push(i);
                }
            }
        }
        let func_info = self.get_func_unsafe_mut(&func.identifier.id);
        for i in args_to_fix {
            func_info.parameters[i].info.ty = None;
        }
        if fix_return_type {
            func_info.name.info.ty = None;
        }
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
