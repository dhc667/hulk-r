use crate::ProgramParser;

#[test]
fn expr_first() {
    let p = ProgramParser::new();
    let answ = p
        .parse(
            "

let a = new BullDog() in {a.sound();};

constant PI: Number = 3.14;

type Dog {
    sound(): String => \"woof!\";
}

type BullDog inherits Dog {
    sound(): String => \"(bull) woof!\";
}

type Cat {
    sound(): String => \"meow\";
}

",
        )
        .unwrap();

    assert_eq!(
        answ.expressions[0]
            .as_let_in()
            .unwrap()
            .assignment
            .identifier
            .id,
        "a"
    );
}

#[test]
fn expr_in_the_middle() {
    let p = ProgramParser::new();
    let answ = p
        .parse(
            "
constant PI: Number = 3.14;

type Dog {
    sound(): String => \"woof!\";
}

let a = new BullDog() in {a.sound();};

type BullDog inherits Dog {
    sound(): String => \"(bull) woof!\";
}

type Cat {
    sound(): String => \"meow\";
}

",
        )
        .unwrap();

    assert_eq!(
        answ.expressions[0]
            .as_let_in()
            .unwrap()
            .assignment
            .identifier
            .id,
        "a"
    );
}

#[test]
fn expr_in_the_end() {
    let p = ProgramParser::new();
    let answ = p
        .parse(
            "
constant PI: Number = 3.14;

type Dog {
    sound(): String => \"woof!\";
}


type BullDog inherits Dog {
    sound(): String => \"(bull) woof!\";
}

type Cat {
    sound(): String => \"meow\";
}

let a = new BullDog() in {a.sound();};
",
        )
        .unwrap();

    assert_eq!(
        answ.expressions[0]
            .as_let_in()
            .unwrap()
            .assignment
            .identifier
            .id,
        "a"
    );
}

#[test]
fn multiple_exprs() {
    let p = ProgramParser::new();
    let answ = p
        .parse(
            "
constant PI: Number = 3.14;


let a = new Dog() in {a.sound();};

type Dog {
    sound(): String => \"woof!\";
}

let a = new Cat() in {a.sound();};

type BullDog inherits Dog {
    sound(): String => \"(bull) woof!\";
}

type Cat {
    sound(): String => \"meow\";
}

let a = new BullDog() in {a.sound();};
",
        )
        .unwrap();

    assert_eq!(answ.expressions.len(), 3);
    assert_eq!(
        answ.expressions[0]
            .as_let_in()
            .unwrap()
            .assignment
            .rhs
            .as_new_expression()
            .unwrap()
            .type_name,
        "Dog"
    );
    assert_eq!(
        answ.expressions[1]
            .as_let_in()
            .unwrap()
            .assignment
            .rhs
            .as_new_expression()
            .unwrap()
            .type_name,
        "Cat"
    );
    assert_eq!(
        answ.expressions[2]
            .as_let_in()
            .unwrap()
            .assignment
            .rhs
            .as_new_expression()
            .unwrap()
            .type_name,
        "BullDog"
    );
}
