use crate::test::lli_interface::lli_f64;

use super::generate_code;

#[test]
fn adding_blocks() {
    let llvm = generate_code(
        "print(
            {
                let x = 3 in (x + 4);
            } + {
                let y = 4 in (y + 5); 
            }
        );",
    );
    println!("{}", llvm);

    let result = lli_f64(&llvm).unwrap();
    let expected = 3.0 + 4.0 + 4.0 + 5.0;

    assert_eq!(result, expected);

}

#[test]
#[should_panic]
fn null_returning_block() {
    let llvm = generate_code(
        "print(
            {
                let x = 3 in (x + 4);
            } + {
                let y = 4 in (y + 5);;
            }
        );",
    );
    println!("{}", llvm);

    let result = lli_f64(&llvm).unwrap();
    let expected = 3.0 + 4.0 + 4.0 + 5.0;

    assert_eq!(result, expected);
}
