use crate::test::lli_interface::lli_f64;

use super::generate_code;

#[test]
fn data_access() {
    let llvm = crate::test::generate_code(
        "
        type Point (a:Number,b:Number) {x=a;y=b;}
        let a = new Point(4,5) in 
        print(
            {
                let x = 3 in (x + 4);
            } + {
                a.x+a.y; 
            }
        );",
    );
    println!("{}", llvm);

    let result = lli_f64(&llvm).unwrap();
    let expected = 3.0 + 4.0 + 4.0 + 5.0;

    assert_eq!(result, expected);
}