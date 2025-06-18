use super::generate_code;
use crate::test::lli_interface::lli_string;
use crate::test::lli_interface::{lli_f64, lli_i1};

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
            type Point (a:Number,b:Boolean) {x=a;y=b;}
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
            type Point (a:Number) {x=a; get(n:Number): Number { return let self = n in self; } }

            let a = new Point(4) in
            let x = a.get(4) in print(x);
            ",
    );
    println!("{}", llvm);

    assert_eq!(lli_f64(&llvm).unwrap(), 4.0);
}

#[test]
fn inherits_test() {
    let llvm = crate::test::generate_code(
        "
        type Point1 (a:Number,b:Number) {x=a;y=b; get(): Number { return self.x; }}
        type Point2 (c:Number,d:Number) inherits Point1(c*2,d) { get3(): Number { return 3; } get2(): Number { return 2; } }
        type Point3 (e:Number,f:Number) inherits Point2(e,f) { a = e;}
        let p = new Point3(5,1) in
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
        type Animal() { g(): Number { return 20; } talk(): Number { return 10; }}
        type Dog() inherits Animal() { g(): Number { return 20; } }
        type Cat() inherits Animal() { talk(): Number { return 30; } }
        let p = new Cat() in
        print(
           p.talk()
        );",
    );
    println!("{}", llvm);

    let result = lli_f64(&llvm).unwrap();
    let expected = 30.0;

    assert_eq!(result, expected);
}

#[test]
fn function_definition_with_return_type_test() {
    let llvm = generate_code(
        "
            type Animal() {talk(): Number { return 10; }}
            type Dog() inherits Animal() { y=7; }
            type Cat() inherits Animal() { x=200; y=300; talk(): Number { return self.talk2(); } talk2(): Number { return 300; } }
            function f(): Animal {
                return new Cat();
            }
            let a = f() in print(a.talk());
            ",
    );
    println!("{}", llvm);

    assert_eq!(lli_f64(&llvm).unwrap(), 300.0);
}

#[test]
fn abc() {
    let llvm = generate_code(
        r#"
            type A() { x=10; f(): Number { self.x; } get_x(): Number { return self.x;}  }
            type B() inherits A() {  h(): Number { return 20; } }
            type C() inherits B() { f(): Number { return 3*self.get_x(); } }

            function g(): B {
                if (true)  {
                return new C();
                } else { return new B(); };
            }

            let a = g() in print(a.f());
        "#,
    );
    println!("{}", llvm);

    let result = lli_f64(&llvm).unwrap();
    let expected = 30.0;

    assert_eq!(result, expected);
}

#[test]
fn test_string() {
    let llvm = generate_code(
        r#"


            let a = "hello world" in print(a);
        "#,
    );
    println!("{}", llvm);

    let result = lli_string(&llvm).unwrap();
    let expected = "hello world";

    assert_eq!(result, expected);
}

#[test]
fn test_string_2() {
    let llvm = generate_code(
        r#"
            let a = "hello world"  in let b = a in print(b);
        "#,
    );
    println!("{}", llvm);

    let result = lli_string(&llvm).unwrap();
    let expected = "hello world";

    assert_eq!(result, expected);
}

#[test]
fn test_josue_name() {
    let llvm = generate_code(
        r#"
            type Person ( name: String,age: Number){
                age=age*2;
                name=name;
                getAge(): Number { return self.age; }
                getName(): String { return self.name; }
            }
            let p = new Person("josue",20) in print(p.getName());

        "#,
    );
    println!("{}", llvm);

    let result = lli_string(&llvm).unwrap();
    let expected = "josue";

    assert_eq!(result, expected);
}

#[test]
fn test_dario_and_josue_name() {
    let llvm = generate_code(
        r#"
            type Person ( name: String,age: Number){
                age=age*2;
                name=name;
                getAge(): Number { return self.age; }
                getName(): String { return self.name; }
            }
            let josue = new Person("josue",20), dario = new Person("dario",20) in print(josue.getName() @ " y " @ dario.getName());

        "#,
    );
    println!("{}", llvm);

    let result = lli_string(&llvm).unwrap();
    let expected = "josue y dario";

    assert_eq!(result, expected);
}

#[test]
fn test_josue_age() {
    let llvm = generate_code(
        r#"
            type Person ( name: String,age: Number){
                age=age*2;
                name=name;
                getAge(): Number { return self.age; }
                getName(): String { return self.name; }
            }
            let p = new Person("josue",20) in print(p.getAge());

        "#,
    );
    println!("{}", llvm);

    let result = lli_f64(&llvm).unwrap();
    let expected = 40.0;

    assert_eq!(result, expected);
}
//
// #[test]
// fn test_string_concatenation() {
//     let llvm = generate_code(
//         r#"
//
//
//             let a = "hello" @ " world" in print(a);
//         "#,
//     );
//     println!("{}", llvm);
//
//     let result = lli_string(&llvm).unwrap();
//     let expected = "hello world";
//
//     assert_eq!(result, expected);
// }
#[test]
fn deep_inheritance_and_override() {
    let llvm = generate_code(
        r#"
            type Base(a: Number) { x = a; getx(): Number { return self.x; } }
            type Mid(a: Number) inherits Base(a + 1) { get(): Number { return self.getx() * 2; } }
            type Leaf(a: Number) inherits Mid(a * 2) { get(): Number { return self.getx() + 100; } }
            let obj = new Leaf(5) in print(obj.get());
        "#,
    );
    println!("{}", llvm);
    let result = lli_f64(&llvm).unwrap();
    // Leaf(5) -> Mid(10) -> Base(11), so x = 11, get() = 11 + 100 = 111
    assert_eq!(result, 111.0);
}

#[test]
fn nested_type_instantiation_and_method_calls() {
    let llvm = generate_code(
        r#"
            type Inner(a: Number) { val = a; get(): Number { return self.val; } }
            type Outer(a: Number) { inner = new Inner(a * 3); getInnerVal(): Number { return self.inner.get(); } }
            let o = new Outer(7) in print(o.getInnerVal());
        "#,
    );
    println!("{}", llvm);
    let result = lli_f64(&llvm).unwrap();
    assert_eq!(result, 21.0);
}

#[test]
fn mutate_fields_and_verify() {
    let llvm = generate_code(
        r#"
            type Counter(start: Number) { value = start; get_value():Number => self.value; inc(): Number { self.value := self.value + 1; return self.value; } }
            let c = new Counter(10) in { c.inc();  c.inc(); print(c.get_value()); };
        "#,
    );
    println!("{}", llvm);
    let result = lli_f64(&llvm).unwrap();
    assert_eq!(result, 12.0);
}

#[test]
fn complex_string_manipulation() {
    let llvm = generate_code(
        r#"
            let a = "foo", b = "bar", c = "baz" in print(a @ "-" @ b @ ":" @ c);
        "#,
    );
    println!("{}", llvm);
    let result = lli_string(&llvm).unwrap();
    assert_eq!(result, "foo-bar:baz");
}

#[test]
fn complex_string_manipulation_2() {
    let llvm = generate_code(
        r#"
            function f(): String {"foobabaz";}

            let a = f() in print(a);

        "#,
    );
    println!("{}", llvm);
    let result = lli_string(&llvm).unwrap();
    assert_eq!(result, "foobabaz");
}

#[test]
fn x1() {
    let llvm = generate_code(
        r#"
            function f(): Number { return let a = 5 in a; }
            let x = f() in print(x);
        "#,
    );
    println!("{}", llvm);
    let result = lli_f64(&llvm).unwrap();
    assert_eq!(result, 5.0);
}

#[test]
fn return_string_from_function() {
    let llvm = generate_code(
        r#"
            function f(): String { return let a = "a" in { a:="hello";  (a @" world");}; }
            let x = f() in print(x);
        "#,
    );
    println!("{}", llvm);
    let result = lli_string(&llvm).unwrap();
    assert_eq!(result, "hello world");
}

#[test]
fn interleaved_number_and_string_operations() {
    let llvm = generate_code(
        r#"
            let n = true,m=false, s = " is the answer and not " in print(n @ s @ m);
        "#,
    );
    println!("{}", llvm);
    let result = lli_string(&llvm).unwrap();
    assert_eq!(result, "true is the answer and not false");
}

#[test]
fn interleaved_number_and_string_operations_with_numbers() {
    let llvm = generate_code(
        r#"
            let n = 1,m=20, s = " is the answer and not " in print(n @ s @ m);
        "#,
    );
    println!("{}", llvm);
    let result = lli_string(&llvm).unwrap();
    assert_eq!(result, "1.000000 is the answer and not 20.000000");
}

#[test]
fn simple_while_2() {
    let llvm = generate_code(
        r#"let x = 1 in {
            while(x < 6) {
                let a = "hello" in {
                    let b = a @@ "world" in {
                     print(b);
                    };
                };
                print(x);
                 x := x + 1;
            };

        };"#,
    );

    println!("{}", llvm);
    assert_eq!(
        lli_string(&llvm).unwrap(),
        "hello world
1.000000
hello world
2.000000
hello world
3.000000
hello world
4.000000
hello world
5.000000"
    );
}

#[test]
fn list_of_numbers() {
    let llvm = generate_code(
        r#"
            function f(): String { return let a = "a" in { a:="hello";  (a @" world");}; }
            let x = f(),y = [1,2,3] in print(x);
        "#,
    );
    println!("{}", llvm);
    let result = lli_string(&llvm).unwrap();
    assert_eq!(result, "hello world");
}

#[test]
fn list_of_bools() {
    let llvm = generate_code(
        r#"
            function f(): Bool { return false; }
            let x = f() in let y = [true,x] in print(x);
        "#,
    );
    println!("{}", llvm);
    let result = lli_i1(&llvm).unwrap();
    assert_eq!(result, false);
}

#[test]
fn list_of_strings_with_indexing() {
    let llvm = generate_code(
        r#"
            function f(): String { return let a = "a" in { a:="hello";  (a @" world");}; }
            let x = f(),y = ["hi",x] in print(y[1]);
        "#,
    );
    println!("{}", llvm);
    let result = lli_string(&llvm).unwrap();
    assert_eq!(result, "hello world");
}

#[test]
fn list_of_types() {
    let llvm = generate_code(
        r#"
            type Person ( name: String,age: Number){
                age=age*2;
                name=name;
                getAge(): Number { return self.age; }
                getName(): String { return self.name; }
            }
            let josue = new Person("josue",20), dario = new Person("dario",20) , list = [josue,dario] in print(josue.getName() @ " y " @ dario.getName());
            
        "#,
    );
    println!("{}", llvm);
    let result = lli_string(&llvm).unwrap();
    assert_eq!(result, "josue y dario");
}

#[test]
fn factorial() {
    let llvm = generate_code(
        r#"
            function factorial(n: Number): Number {
                if (n == 0) {
                    return 1;
                } else {
                    return n * factorial(n - 1);
                };
            }
            let x = factorial(5) in print(x);
            "#,
    );
    println!("{}", llvm);
    let result = lli_f64(&llvm).unwrap();
    assert_eq!(result, 120.0);
}
