use super::generate_code;
use crate::test::lli_interface::{lli_f64, lli_i1};

#[test]
fn boolean_operations1() {
    let llvm = generate_code(
        "let x = 
            if (if(5<4) { true;} else {false;})
                { 2; }
            else { 
                let x = 2 in {
                    1;
                };
            }
        in print(x);",
    );
    println!("{}", llvm);

    let result = lli_f64(&llvm).unwrap();
    let expected = 1.0;

    assert_eq!(result, expected);
}

#[test]
fn boolean_operations2() {
    let llvm = generate_code(
        "let x =
            if (if(5<4) { true;} else {false;})
                { true; }
            else {
                {
                    false;
                };
            }
        in print(x);",
    );
    println!("{}", llvm);

    let result = lli_i1(&llvm).unwrap();
    let expected = false;

    assert_eq!(result, expected);
}

#[test]
fn test_true_literal() {
    let llvm = generate_code("print(true);");
    println!("{}", llvm);

    let true_literal = lli_i1(&llvm).unwrap();
    assert_eq!(true_literal, true);
}

#[test]
fn test_true_result() {
    let llvm = generate_code("print(true || true && false);");
    println!("{}", llvm);

    let true_result = lli_i1(&llvm).unwrap();
    assert_eq!(true_result, true);
}

#[test]
fn test_false_literal() {
    let llvm = generate_code("print(false);");
    println!("{}", llvm);

    let false_literal = lli_i1(&llvm).unwrap();
    assert_eq!(false_literal, false);
}

#[test]
fn test_false_result() {
    let llvm = generate_code("print(false || true && false);");
    println!("{}", llvm);

    let false_result = lli_i1(&llvm).unwrap();
    assert_eq!(false_result, false);
}

#[test]
fn test_composite_boolean_expression() {
    let llvm = generate_code(
        "
        let x = 5, y = 10, z = 15 in
        print(((x < y) && (y < z) || (x > z)) == true);",
    );

    println!("{}", llvm);

    let result = lli_i1(&llvm).unwrap();
    let expected = true;
    assert_eq!(result, expected);
}

#[test]
fn test_composite_boolean_expression2() {
    let llvm = generate_code(
        "
        print(5 < 3 || 2 == 2 && true == false || 1 != 0);
        ",
    );

    println!("{}", llvm);

    let result = lli_i1(&llvm).unwrap();
    let expected = true;
    assert_eq!(result, expected);
}
