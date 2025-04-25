use crate::grammar;
use ast;

#[test]
fn detects_single_semicolon_terminated_block() {
    let p = grammar::AtomParser::new();

    let answ = p.parse(
        "{
a + b;
c + d;;;
x - 4;
}",
    );

    if let Ok(ast::Atom::Block(block)) = answ {
        assert_eq!(block.expression_list.expressions.len(), 3);
        assert!(!block.expression_list.multiple_semicolon_terminated);
        assert_eq!(
            block.expression_list.expressions[0]
                .as_bin_op()
                .unwrap()
                .lhs
                .as_atom()
                .unwrap()
                .as_variable()
                .unwrap()
                .identifier
                .id,
            "a"
        );
        assert_eq!(
            block.expression_list.expressions[1]
                .as_bin_op()
                .unwrap()
                .lhs
                .as_atom()
                .unwrap()
                .as_variable()
                .unwrap()
                .identifier
                .id,
            "c"
        );
        assert_eq!(
            block.expression_list.expressions[2]
                .as_bin_op()
                .unwrap()
                .lhs
                .as_atom()
                .unwrap()
                .as_variable()
                .unwrap()
                .identifier
                .id,
            "x"
        );
    } else {
        panic!("Expected Block");
    }
}

#[test]
fn detects_multiple_semicolon_terminated_block() {
    let p = grammar::AtomParser::new();

    let answ = p.parse(
        "{
a + b;
c + d;
x - 4 + 6 / (2 + 3 - x);;
}",
    );

    if let Ok(ast::Atom::Block(block)) = answ {
        assert_eq!(block.expression_list.expressions.len(), 3);
        assert!(block.expression_list.multiple_semicolon_terminated);
        assert_eq!(
            block.expression_list.expressions[0]
                .as_bin_op()
                .unwrap()
                .lhs
                .as_atom()
                .unwrap()
                .as_variable()
                .unwrap()
                .identifier
                .id,
            "a"
        );
        assert_eq!(
            block.expression_list.expressions[1]
                .as_bin_op()
                .unwrap()
                .lhs
                .as_atom()
                .unwrap()
                .as_variable()
                .unwrap()
                .identifier
                .id,
            "c"
        );
        assert_eq!(
            block.expression_list.expressions[2]
                .as_bin_op()
                .unwrap()
                .rhs
                .as_bin_op()
                .unwrap()
                .rhs
                .as_atom()
                .unwrap()
                .as_grouped_expression()
                .unwrap()
                .as_bin_op()
                .unwrap()
                .rhs
                .as_atom()
                .unwrap()
                .as_variable()
                .unwrap()
                .identifier
                .id,
            "x"
        );
    } else {
        panic!("Expected Block");
    }
}
