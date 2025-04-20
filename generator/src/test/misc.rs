use crate::test::{generate_code, lli_interface::call_lli};

#[test]
fn fibonacci_numbers() {
    let llvm = generate_code(
        "let x = 10 in {
          let a = 0, b = 0, fib = 1 in {
              while(x - 1) {
                  a := b;
                  b := fib;
                  fib := a + b;
                  print(fib);
                  x:= x -1 ;
              };
          };
      };",
    );
    println!("{}", llvm);

    println!("{}", call_lli(&llvm).unwrap())
}
