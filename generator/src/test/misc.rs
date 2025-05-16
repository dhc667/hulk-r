use crate::test::{generate_code, lli_interface::lli_f64};

#[test]
fn fibonacci_numbers() {
    let llvm = generate_code(
        "let x = 10 in {
          let a = 0, b = 0, fib = 1 in {
              while(x != 1) {
                  a := b;
                  b := fib;
                  fib := a + b;
                  x:= x -1 ;
              };
              print(fib);
          };
      };",
    );
    println!("{}", llvm);

    assert_eq!(lli_f64(&llvm).unwrap(), 55.0);
}
