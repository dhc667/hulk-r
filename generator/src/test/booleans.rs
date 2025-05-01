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