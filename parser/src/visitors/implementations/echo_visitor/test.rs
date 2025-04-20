use crate::{grammar::ExpressionListParser, visitors::visitable::Visitable};
use super::EchoVisitor;


#[test]
fn echoes_simple_expression() {
    let p = ExpressionListParser::new();

    let mut answ = p.parse("1 + 2 + 3 * 4;2+2;").unwrap();

    let mut echoer = EchoVisitor::new();

    let result = answ.accept(&mut echoer);

    assert_eq!(result, "((1 + 2) + (3 * 4)); (2 + 2);");

    let result2 = answ.accept(&mut echoer);

    assert_eq!(result2, result);
}

#[test]
fn echoes_multiple_semicolon_terminated_exp_list() {
    let p = ExpressionListParser::new();

    let mut answ = p.parse("1 + 2 + 3 * 4; 2+2;;;;").unwrap();

    let mut echoer = EchoVisitor::new();

    let result = answ.accept(&mut echoer);

    assert_eq!(result, "((1 + 2) + (3 * 4)); (2 + 2);;");

    let result2 = answ.accept(&mut echoer);

    assert_eq!(result2, result);
}

#[test]
fn echoes_if_statement() {
    let p = ExpressionListParser::new();

    let mut answ = p.parse("if (x + 3) { print(h); x + 2;; } else d;").unwrap();

    let mut echoer = EchoVisitor::new();

    let result = answ.accept(&mut echoer);

    assert_eq!(result, "if ((x + 3)) { print(h); (x + 2);; } else d;");

    let result2 = answ.accept(&mut echoer);

    assert_eq!(result2, result);
}

#[test]
fn echoes_let_in_statement() {
    let p = ExpressionListParser::new();

    let mut answ = p.parse("let x = 1 + 2, y = x + 2, z = 3 in {1 + 2 + 3 / 4;};").unwrap();

    let mut echoer = EchoVisitor::new();

    let result = answ.accept(&mut echoer);

    assert_eq!(result, "let x = (1 + 2) in let y = (x + 2) in let z = 3 in { ((1 + 2) + (3 / 4)); };")

}
