use crate::test::lli_interface::lli_f64;

use super::generate_code;

#[test]
fn simple_dassignment () {
    let llvm = generate_code(
        "let x = 3 in {
            x := x + 1;
            print(x);
        };",
    );

    println!("{}", llvm);
    let result = lli_f64(&llvm).unwrap();

    let expected = 3.0 + 1.0;
    assert_eq!(result, expected);
}

#[test]
fn block_resistant_dassignment () {
    let llvm = generate_code(
        "let x = 3 in {
            let y = 6 in {
                x := y;
            };
            print(x); 
        };",
    );

    println!("{}", llvm);
    let result = lli_f64(&llvm).unwrap();

    let expected = 6.0;
    assert_eq!(result, expected);
}
