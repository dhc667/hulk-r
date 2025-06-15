use crate::test::lli_interface::{lli_f64, lli_string};

use super::generate_code;

#[test]
fn simple_while() {
    let llvm = generate_code(
        "let x = 10 in {
            while(x != 1) {
                x := x - 1;
            };
            print(x);
        };",
    );

    println!("{}", llvm);
    assert_eq!(lli_f64(&llvm).unwrap(), 1.0);
}

// #[test]
// fn simple_for() {
//     let llvm = generate_code(
//         "
//         for (x in [1,2,3]) {
//             print(x);
//         };
        
//         ",
//     );

//     println!("{}", llvm);
//     assert_eq!(lli_f64(&llvm).unwrap(), 1.0);
// }

#[test]
fn simple_while_2() {
    let llvm = generate_code(
        "
        let i = 0 , array = [1,2,3,4,5,6,7] in {
            while(i < 3) {
                print(array[i]);
                i := i + 1;
            };

        };",
    );

    println!("{}", llvm);
    assert_eq!(lli_string(&llvm).unwrap(), "1.000000
2.000000
3.000000");
}


