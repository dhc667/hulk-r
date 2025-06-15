use crate::test::expression_parser::ExpressionParser;

#[test]
fn simple_list_literal() {
    let p = ExpressionParser::new();

    let answ = p.parse("[1, 2, 3]").unwrap();

    assert_eq!(
        answ.as_list_literal().unwrap().elements[1]
            .as_number_literal()
            .unwrap()
            .value,
        2.0
    );

    assert_eq!(
        answ.as_list_literal().unwrap().elements[2]
            .as_number_literal()
            .unwrap()
            .value,
        3.0
    );
}

#[test]
fn add_list_literals() {
    let p = ExpressionParser::new();

    let answ = p.parse("[1, 2, 3] + [1, 2]").unwrap();

    assert_eq!(
        answ.as_bin_op()
            .unwrap()
            .rhs
            .as_list_literal()
            .unwrap()
            .elements[1]
            .as_number_literal()
            .unwrap()
            .value,
        2.0
    );

    assert_eq!(
        answ.as_bin_op()
            .unwrap()
            .lhs
            .as_list_literal()
            .unwrap()
            .elements[0]
            .as_number_literal()
            .unwrap()
            .value,
        1.0
    )
}

#[test]
fn index_list_literal() {
    let p = ExpressionParser::new();

    let answ = p.parse("[[1], [2], [3]][1][0]").unwrap();

    assert_eq!(
        answ.as_list_indexing()
            .unwrap()
            .list
            .as_list_indexing()
            .unwrap()
            .list
            .as_list_literal()
            .unwrap()
            .elements[1]
            .as_list_literal()
            .unwrap()
            .elements[0]
            .as_number_literal()
            .unwrap()
            .value,
        2.0
    );

    assert_eq!(
        answ.as_list_indexing()
            .unwrap()
            .index
            .as_number_literal()
            .unwrap()
            .value,
        0.0
    )
}

#[test]
fn complex_expresion_element() {
    let p = ExpressionParser::new();

    let answ = p
        .parse("[a + b, {1 + 2 + 3; 2 + 4;}, let a = 3 in a] + [1]")
        .unwrap();

    assert_eq!(
        answ.as_bin_op()
            .unwrap()
            .lhs
            .as_list_literal()
            .unwrap()
            .elements[1]
            .as_block()
            .unwrap()
            .body_items[1]
            .as_expression()
            .unwrap()
            .as_bin_op()
            .unwrap()
            .lhs
            .as_number_literal()
            .unwrap()
            .value,
        2.0
    );

    assert_eq!(
        answ.as_bin_op()
            .unwrap()
            .lhs
            .as_list_literal()
            .unwrap()
            .elements[2]
            .as_let_in()
            .unwrap()
            .assignment
            .identifier
            .id,
        "a"
    )
}

#[test]
fn complex_index_expression() {
    let p = ExpressionParser::new();

    let answ = p.parse("[1, a + b][a + c]").unwrap();

    assert_eq!(
        answ.as_list_indexing()
            .unwrap()
            .index
            .as_bin_op()
            .unwrap()
            .lhs
            .as_variable()
            .unwrap()
            .id,
        "a"
    )
}

#[test]
fn indexing_block() {
    let p = ExpressionParser::new();

    let answ = p.parse("{let x = 3 in x; [1];}[0]").unwrap();

    assert_eq!(
        answ.as_list_indexing()
            .unwrap()
            .list
            .as_block()
            .unwrap()
            .body_items[1]
            .as_expression()
            .unwrap()
            .as_list_literal()
            .unwrap()
            .elements[0]
            .as_number_literal()
            .unwrap()
            .value,
        1.0
    );
}

#[test]
fn parenthesis_list_indexing() {
    let p = ExpressionParser::new();

    let answ = p.parse("([1] + [1])[0]").unwrap();

    assert_eq!(
        answ.as_list_indexing()
            .unwrap()
            .list
            .as_bin_op()
            .unwrap()
            .rhs
            .as_list_literal()
            .unwrap()
            .elements[0]
            .as_number_literal()
            .unwrap()
            .value,
        1.0
    );
}
