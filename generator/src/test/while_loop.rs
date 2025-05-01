use crate::test::lli_interface::lli_f64;

use super::generate_code;

#[test]
fn simple() {
    let llvm = generate_code(
        "let x = 10 in {
            while(x - 1) {
                x := x - 1;
            };
            print(x);
        };",
    );

    println!("{}", llvm);
    assert_eq!(lli_f64(&llvm).unwrap(), 1.0);
}
