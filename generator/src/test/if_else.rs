use crate::test::{generate_code, lli_interface::lli_f64};

#[test]
fn test_if_else() {
    let llvm = generate_code(
        "let x = 
            if (false) 
                { 1; } 
            else { 
                let x = 2 in {
                    x := x + 1;
                    x := x + 1;
                    x;
                };
            }
        in print(x + 1);",
    );
    println!("{}", llvm);

    assert_eq!(lli_f64(&llvm).unwrap(), 5.0);
}

#[test]
#[should_panic(expected = "Variable x not found")]
fn using_variable_in_definition() {
    let llvm = generate_code(
        "let x = 
            if (false) 
                { 1; }
            else { 
                x := x + 1;
                x := x + 1;
                x;
            }
        in print(x + 1);",
    );
    println!("{}", llvm);

    assert_eq!(lli_f64(&llvm).unwrap(), 5.0);
}
