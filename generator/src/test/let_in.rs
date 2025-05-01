use super::{generate_code, lli_interface::lli_f64};

#[test]
fn simple_let_in() {
    let llvm = generate_code("let x = 3 in print(7 + x);");

    println!("{}", llvm);
    let result = lli_f64(&llvm).unwrap();

    let expected = 7.0 + 3.0;
    assert_eq!(result, expected);
}

#[test]
fn shadow_let_in() {
    let llvm = generate_code("let x = 3 in let x = 4 in print(x);");

    println!("{}", llvm);
    let result = lli_f64(&llvm).unwrap();

    let expected = 4.0;
    assert_eq!(result, expected);
}

#[test]
fn composite_let_in() {
    let llvm = generate_code("let x = 3 in let y = 4 in print(x + let z = 5 in (y + z));");

    println!("{}", llvm);
    let result = lli_f64(&llvm).unwrap();

    let expected = 3.0 + 4.0 + 5.0;
    assert_eq!(result, expected);
}

#[test]
fn several_let_in() {
    let llvm = generate_code("let x = 3, y = 4, x = x + y, z = y + 5 in print(x + z);");

    println!("{}", llvm);
    let result = lli_f64(&llvm).unwrap();

    let expected = 3.0 + 4.0 + 4.0 + 5.0;
    assert_eq!(result, expected);
}

#[test]
#[should_panic(expected = "Variable x not found")]
fn not_found_let_in() {
    let llvm = generate_code("let y = 3 in print(x + y);");

    println!("{}", llvm);
    let result = lli_f64(&llvm).unwrap();

    let expected = 3.0 + 4.0 + 4.0 + 5.0;
    assert_eq!(result, expected);
}

#[test]
#[should_panic(expected = "Variable x not found")]
fn not_found_let_in_2() {
    let llvm = generate_code("let x = 3 in { x := x + 1; }; let y = 4 in print(x + y);");

    println!("{}", llvm);
    let result = lli_f64(&llvm).unwrap();

    let expected = 3.0 + 4.0 + 4.0 + 5.0;
    assert_eq!(result, expected);
}
