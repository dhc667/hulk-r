use crate::test::lli_interface::lli_f64;

use super::generate_code;

#[test]
fn data_access() {
    let llvm = crate::test::generate_code(
        "
        type Point1 (a:Number,b:Number) {x=a;y=b;}
        type Point2 (a:Number,b:Point1) {x=a;y=b;}
        let p = new Point1(1,5) in 
        let q = new Point2(4,p) in
        print(
            {
                let x = 3 in (x + 4);
            } + {
                q.y.x;
            }
        );",
    );
    println!("{}", llvm);

    let result = lli_f64(&llvm).unwrap();
    let expected = 3.0 + 4.0 + 1.0;

    assert_eq!(result, expected);
}

#[test]
fn data_access_bool() {
    let llvm = generate_code(
        "
            type Point (a:Number,b:Boolean) {x=a;y=b;}
            let a = new Point(4,false) in
            let x =
            if (a.y)
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
fn function_definition() {
    let llvm = generate_code(
        "
            function f(x: Number): Number { return x; }
            function g(x: Number): Number { return f(x); }
            type Point (a:Number,b:Number) {x=a;y=b;}
            let a = new Point(4,false) in
            let x = g(4) in print(g(x));
            ",
    );
    println!("{}", llvm);

    assert_eq!(lli_f64(&llvm).unwrap(), 4.0);
}
#[test]
fn function_definition_with_type_parameter() {
    let llvm = generate_code(
        "
            type Point (a:Number,b:Number) {x=a;y=b;}
            function f(x: Boolean,y:Point): Number { return if (x) {
                y.x;
            }
            else {
                y.y;
            }; }
            let a = new Point(4,10) in
            let x = f(false,a) in print(x);
            ",
    );
    println!("{}", llvm);

    assert_eq!(lli_f64(&llvm).unwrap(), 10.0);
}

#[test]
fn function_definition_in_type() {
    let llvm = generate_code(
        "
            type Point (a:Number) {x=a; get(): Number { return self.x; } }

            let a = new Point(4) in
            let x = a.get() in print(x);
            ",
    );
    println!("{}", llvm);

    assert_eq!(lli_f64(&llvm).unwrap(), 4.0);
}

#[test]
fn two_function_in_two_types_definitions() {
    let llvm = generate_code(
        "
            type Point (a:Number) {x=a; get(): Number { return self.x; } gettimes(n:Number): Number {return self.x*n;} }
            type Point2 (a:Number) {x=a; gettimes(): Number {return self.x*2;} }
            let a = new Point2(4) in
            let x = a.gettimes() in print(x);
            ",
    );
    println!("{}", llvm);

    assert_eq!(lli_f64(&llvm).unwrap(), 8.0);
}
#[test]
fn shadow_to_self() {
    let llvm = generate_code(
        "
            type Point (a:Number) {x=a; get(): Number { return let self = 4 in self; } }

            let a = new Point(4) in
            let x = a.get() in print(x);
            ",
    );
    println!("{}", llvm);

    assert_eq!(lli_f64(&llvm).unwrap(), 4.0);
}

#[test]
fn inherits_test() {
    let llvm = crate::test::generate_code(
        "
        type Point1 (a:Number,b:Number) {x=a;y=b; get(): Number { return self.y; }}
        type Point2 (c:Number,d:Number) inherits Point1(c,d*2) { get3(): Number { return 3; } get2(): Number { return 2; } }
        type Point3 (e:Number,f:Number) inherits Point2(e,f) {get(): Number { return 12; } }
        let p = new Point3(1,5) in
        print(
           p.get()
        );",
    );
    println!("{}", llvm);

    let result = lli_f64(&llvm).unwrap();
    let expected = 12.0;

    assert_eq!(result, expected);
}

#[test]
fn inherits_3_test() {
    let llvm = crate::test::generate_code(
        "
        type Point1 (a:Number,b:Number) {x=a;y=b; get(): Number { return self.y; }}
        type Point2 (c:Number,d:Number) inherits Point1(c,d*2) {z=c; get3(): Number { return 3; } }
        let p = new Point2(1,5) in
        print(
           p.get()
        );",
    );
    println!("{}", llvm);

    let result = lli_f64(&llvm).unwrap();
    let expected = 10.0;

    assert_eq!(result, expected);
}
#[test]
fn inherits_2_test() {
    let llvm = crate::test::generate_code(
        "
        type Animal() {talk(): Number { return 10; }}
        type Dog() inherits Animal() { talk(): Number { return 20; } }
        type Cat() inherits Animal() { talk(): Number { return 30; } }
        let p = new Dog() in
        print(
           p.talk()
        );",
    );
    println!("{}", llvm);

    let result = lli_f64(&llvm).unwrap();
    let expected = 20.0;

    assert_eq!(result, expected);
}
