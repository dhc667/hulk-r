use crate::test::lli_interface::lli_f64;

use super::generate_code;



#[test]
fn simple_operation() {
    let llvm = generate_code("print(2 + 2 + 3 + 7 * 2);");
    println!("{}", llvm);
    let expected = 2.0 + 2.0 + 3.0 + 7.0 * 2.0;
    assert_eq!(lli_f64(&llvm).unwrap(), expected);
}
