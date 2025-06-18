use ast::{VisitableDefinition, VisitableExpression};

use crate::visitor::{GeneratorVisitor, GlobalDefinitionVisitor};

pub struct CodeGenerator {}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_code_from_program_ast(self, node: &mut ast::Program) -> String {
        let mut generator = GeneratorVisitor::new();
        let mut global_definition_visitor = GlobalDefinitionVisitor::new();
        let mut program = generator.instantiate_global_print_helpers();

        for definition in &mut node.definitions {
            let _definition_result = definition.accept(&mut global_definition_visitor);
        }

        generator.functions_args_types = global_definition_visitor.functions_args_types.clone();
        generator.inherits = global_definition_visitor.inherits.clone();
        generator.type_members_types = global_definition_visitor.type_members_types.clone();
        generator.function_member_def_from_type_and_name = global_definition_visitor
            .function_member_def_from_type_and_name
            .clone();
        generator.constructor_args_types = global_definition_visitor.constructor_args_types.clone();
        generator.function_member_names = global_definition_visitor.function_member_names.clone();
        generator.original_type_for_definition = global_definition_visitor
            .original_type_for_definition
            .clone();
        generator.type_members_ids = global_definition_visitor.type_members_ids.clone();
        generator.function_member_signature_types = global_definition_visitor
            .function_member_signature_types
            .clone();

        let mut definitions_code = String::new();
        for definition in &mut node.definitions {
            let definition_result = definition.accept(&mut generator);
            definitions_code += &definition_result.preamble;
        }

        let mut structs_code = String::new();
        for code in generator.general_definitions.iter() {
            structs_code += code;
            structs_code += "\n";
        }

        let mut expressions_code = String::new();
        if !node.expressions.is_empty() {
            for expr in node.expressions.iter_mut() {
                let inner = expr.accept(&mut generator);
                expressions_code += &inner.preamble;
            }
        }

        let mut global_string = String::new();
        for string_global in generator.string_constants.iter() {
            global_string.push_str(&format!("{}\n", string_global.clone()));
        }
        let global_str: &str = global_string.as_str();
        program += global_str;
        program += &structs_code;
        program += &definitions_code;
        program += "define i32 @main() {\nentry:\n";
        program += &expressions_code;
        program += "\nret i32 0\n}\n";

        program
    }
}
