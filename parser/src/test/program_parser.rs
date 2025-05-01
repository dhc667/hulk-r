use crate::grammar::ProgramParser;

#[test]
fn parses_simple_program() {
    let p = ProgramParser::new();

    let result = p.parse("2 + 2; print(2);").unwrap();

    assert_eq!(result.expression_list.expressions.len(), 2);

    let first_expression = &result.expression_list.expressions[0];
    assert_eq!(
        first_expression
            .as_bin_op()
            .unwrap()
            .lhs
            .as_atom()
            .unwrap()
            .as_number_literal()
            .unwrap()
            .value,
        2.0
    );
}
