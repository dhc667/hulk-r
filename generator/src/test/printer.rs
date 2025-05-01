use crate::test::{generate_code, lli_interface::call_lli};

#[test]
fn printer() {
    let llvm = generate_code(
        "let x = 10 in {
            while(x < 1) {
                x := x - 1;
                print(x);
            };
            print(x);
        };",
    );
    println!("{}", llvm);

    println!("{}", call_lli(&llvm).unwrap())
}
