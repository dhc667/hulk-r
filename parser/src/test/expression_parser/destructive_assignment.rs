use crate::grammar::ExpressionParser;

#[test]
fn simple_dassignment() {
    let p = ExpressionParser::new();

    let answ = p.parse("a := 3").unwrap();

    assert_eq!(
        answ.as_destructive_assignment()
            .unwrap()
            .lhs
            .as_variable()
            .unwrap()
            .id,
        "a"
    );
    assert_eq!(
        answ.as_destructive_assignment()
            .unwrap()
            .rhs
            .as_number_literal()
            .unwrap()
            .value,
        3.0
    );
}

#[test]
fn data_member_dassignment() {
    let p = ExpressionParser::new();

    let answ = p.parse("o.m := 3").unwrap();

    assert_eq!(
        answ.as_destructive_assignment()
            .unwrap()
            .lhs
            .as_data_member_access()
            .unwrap()
            .object
            .as_variable()
            .unwrap()
            .id,
        "o"
    );
    assert_eq!(
        answ.as_destructive_assignment()
            .unwrap()
            .lhs
            .as_data_member_access()
            .unwrap()
            .member
            .id,
        "m"
    );
    assert_eq!(
        answ.as_destructive_assignment()
            .unwrap()
            .rhs
            .as_number_literal()
            .unwrap()
            .value,
        3.0
    );
}

#[test]
fn complex_member_reassignment() {
    let p = ExpressionParser::new();

    let answ = p.parse("a.b.get_c().d := w").unwrap();

    assert_eq!(
        answ.as_destructive_assignment()
            .unwrap()
            .lhs
            .as_data_member_access()
            .unwrap()
            .object
            .as_function_member_access()
            .unwrap()
            .member
            .identifier
            .id,
        "get_c"
    )
}
