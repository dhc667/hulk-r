use crate::ProgramParser;

#[test]
fn main_first() {
    let p = ProgramParser::new();
    let answ = p
        .parse(
            "

let a = BullDog() in {a.sound();};

constant PI: Number = 3.14;

protocol Animal {
    sound(): String; 
}

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
        answ.main_expression
            .as_let_in()
            .unwrap()
            .assignment
            .identifier
            .id,
        "a"
    );
}

#[test]
fn main_in_the_middle() {
    let p = ProgramParser::new();
    let answ = p
        .parse(
            "
constant PI: Number = 3.14;

protocol Animal {
    sound(): String; 
}

type Dog {
    sound(): String => \"woof!\";
}

let a = BullDog() in {a.sound();};

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
        answ.main_expression
            .as_let_in()
            .unwrap()
            .assignment
            .identifier
            .id,
        "a"
    );
}

#[test]
fn main_in_the_end() {
    let p = ProgramParser::new();
    let answ = p
        .parse(
            "
constant PI: Number = 3.14;

protocol Animal {
    sound(): String; 
}

type Dog {
    sound(): String => \"woof!\";
}


type BullDog inherits Dog {
    sound(): String => \"(bull) woof!\";
}

type Cat {
    sound(): String => \"meow\";
}

let a = BullDog() in {a.sound();};
",
        )
        .unwrap();

    assert_eq!(
        answ.main_expression
            .as_let_in()
            .unwrap()
            .assignment
            .identifier
            .id,
        "a"
    );
}
