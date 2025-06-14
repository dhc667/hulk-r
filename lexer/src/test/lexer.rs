use std::vec;

use error_handler::error_handler::ErrorHandler;

use crate::lexer_generator::{lexer::Lexer, rule::Rule};

#[test]
pub fn lex_some_tokens() {
    let rules = vec![
        Rule::new("A".to_string(), r"a".to_string()),
        Rule::new("B".to_string(), r"b".to_string()),
        Rule::new_skip("WhiteSpace".to_string(), r"\s+".to_string()),
    ];

    let lexer = Lexer::new(rules);

    let result = lexer.split("aab bba");

    let tokens = result.ok().unwrap();
    assert_eq!(tokens.len(), 6);
    assert_eq!(
        tokens.iter().map(|t| t.ty.clone()).collect::<Vec<_>>(),
        vec![
            "A".to_string(),
            "A".to_string(),
            "B".to_string(),
            "B".to_string(),
            "B".to_string(),
            "A".to_string(),
        ]
    )
}

#[test]
pub fn lex_some_tokens2() {
    let rules = vec![
        Rule::new("A".to_string(), r"a".to_string()),
        Rule::new("B".to_string(), r"b".to_string()),
        Rule::new("C".to_string(), r"c".to_string()),
        Rule::new_skip("WhiteSpace".to_string(), r"\s+".to_string()),
    ];

    let lexer = Lexer::new(rules);

    let result = lexer.split("aab bba ccc");

    let tokens = result.ok().unwrap();
    assert_eq!(tokens.len(), 9);
    assert_eq!(
        tokens.iter().map(|t| t.ty.clone()).collect::<Vec<_>>(),
        vec![
            "A".to_string(),
            "A".to_string(),
            "B".to_string(),
            "B".to_string(),
            "B".to_string(),
            "A".to_string(),
            "C".to_string(),
            "C".to_string(),
            "C".to_string(),
        ]
    )
}

#[test]
pub fn resolve_ambiguity() {
    let rules = vec![
        Rule::new("1".to_string(), r"a".to_string()),
        Rule::new("2".to_string(), r"abb".to_string()),
        Rule::new("3".to_string(), r"a*b+".to_string()),
        Rule::new_skip("WhiteSpace".to_string(), r"\s+".to_string()),
    ];

    let lexer = Lexer::new(rules);

    let result = lexer.split("aab ab");

    let tokens = result.ok().unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(
        tokens.iter().map(|t| t.ty.clone()).collect::<Vec<_>>(),
        vec!["3".to_string(), "3".to_string(),]
    )
}

#[test]
pub fn resolve_ambiguity2() {
    let rules = vec![
        Rule::new("1".to_string(), r"a".to_string()),
        Rule::new("2".to_string(), r"abb".to_string()),
        Rule::new("3".to_string(), r"a*b+".to_string()),
        Rule::new_skip("WhiteSpace".to_string(), r"\s+".to_string()),
    ];
    let lexer = Lexer::new(rules);
    let result = lexer.split("aab ab abb");

    let tokens = result.ok().unwrap();
    assert_eq!(tokens.len(), 3);
    assert_eq!(
        tokens.iter().map(|t| t.ty.clone()).collect::<Vec<_>>(),
        vec!["3".to_string(), "3".to_string(), "2".to_string()]
    )
}

#[test]
pub fn edge_case() {
    let rules = vec![
        Rule::new("1".to_string(), r"a".to_string()),
        Rule::new("2".to_string(), r"a*b".to_string()),
    ];

    let str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    let lexer = Lexer::new(rules);
    let result = lexer.split(str);

    let tokens = result.ok().unwrap();
    assert_eq!(tokens.len(), str.len());
    assert_eq!(
        tokens.iter().map(|t| t.ty.clone()).collect::<Vec<_>>(),
        (0..=(str.len() - 1))
            .into_iter()
            .map(|_| "1".to_string())
            .collect::<Vec<_>>()
    )
}

#[test]
pub fn unrecognized_character() {
    let rules = vec![
        Rule::new("A".to_string(), r"a".to_string()),
        Rule::new("B".to_string(), r"b".to_string()),
        Rule::new_skip("WhiteSpace".to_string(), r"\s+".to_string()),
    ];

    let program_text = "aab bba ccc";

    let lexer = Lexer::new(rules);

    let mut error_handler = ErrorHandler::new(program_text);
    let errors = lexer.split(program_text).err().unwrap().1;

    error_handler.extend_errors(errors);

    assert_eq!(error_handler.errors.len(), 1);
    assert_eq!(
        error_handler.get_raw_errors(),
        vec!["Invalid character `c`"]
    );
}

#[test]
pub fn error_recovery() {
    let rules = vec![
        Rule::new("A".to_string(), r"a".to_string()),
        Rule::new("B".to_string(), r"b".to_string()),
        Rule::new_skip("WhiteSpace".to_string(), r"\s+".to_string()),
    ];

    let lexer = Lexer::new(rules);

    let program_text = "aab bba caac";

    let mut error_handler = ErrorHandler::new(program_text);
    let errors = lexer.split(program_text).err().unwrap().1;

    error_handler.extend_errors(errors);

    assert_eq!(error_handler.errors.len(), 2);
    assert_eq!(
        error_handler.get_raw_errors(),
        vec!["Invalid character `c`", "Invalid character `c`"]
    );
}

#[test]
pub fn lex_hulk_line() {
    let rules = vec![
        Rule::new("EQUAL".to_string(), r"=".to_string()),
        Rule::new("LET".to_string(), r"let".to_string()),
        Rule::new("IN".to_string(), r"in".to_string()),
        Rule::new("SEMICOLON".to_string(), r";".to_string()),
        Rule::new("COLON".to_string(), r":".to_string()),
        Rule::new("NUMBER".to_string(), r"[0-9]+".to_string()),
        Rule::new(
            "IDENTIFIER".to_string(),
            r"(_|[a-zA-Z])(_|[a-z0-9A-Z])*".to_string(),
        ),
        Rule::new_skip("WhiteSpace".to_string(), r"\s+".to_string()),
    ];

    let lexer = Lexer::new(rules);
    let result = lexer.split("let let_var: Number = 5 in x;");

    let tokens = result.ok().unwrap();
    assert_eq!(tokens.len(), 9);
    assert_eq!(
        tokens.iter().map(|t| t.ty.clone()).collect::<Vec<_>>(),
        vec![
            "LET".to_string(),
            "IDENTIFIER".to_string(),
            "COLON".to_string(),
            "IDENTIFIER".to_string(),
            "EQUAL".to_string(),
            "NUMBER".to_string(),
            "IN".to_string(),
            "IDENTIFIER".to_string(),
            "SEMICOLON".to_string()
        ]
    );
}

#[test]
pub fn line_error() {
    let rules = vec![
        Rule::new("A".to_string(), r"a".to_string()),
        Rule::new_skip("WhiteSpace".to_string(), r"(\s|\n)+".to_string()),
    ];

    let lexer = Lexer::new(rules);
    let program_text = "a\nb\nc";

    let mut error_handler = ErrorHandler::new(program_text);
    let errors = lexer.split(program_text).err().unwrap().1;

    error_handler.extend_errors(errors);

    assert_eq!(error_handler.errors.len(), 2);
    assert_eq!(
        error_handler.get_raw_errors(),
        vec!["Invalid character `b`", "Invalid character `c`"]
    );
}

#[test]
pub fn lex_some_python() {
    let rules = vec![
        Rule::new("DEF", r"def".to_string()),
        Rule::new("RETURN", r"return".to_string()),
        Rule::new("LPAREN", r"\(".to_string()),
        Rule::new("RPAREN", r"\)".to_string()),
        Rule::new("COMMA", r",".to_string()),
        Rule::new("COLON", r":".to_string()),
        Rule::new("EQUAL", r"=".to_string()),
        Rule::new("PLUS", r"\+".to_string()),
        Rule::new("MINUS", r"\-".to_string()),
        Rule::new(
            "NUMBER",
            r"(\+|\-)?([0-9]+(\.[0-9]*)?|[0-9]*\.[0-9]+)".to_string(),
        ),
        Rule::new("IDENTIFIER", r"(_|[a-zA-Z])(_|[a-z0-9A-Z])*".to_string()),
        Rule::new_skip("WhiteSpace", r"(\s|\t|\n)+".to_string()),
    ];

    let input = r"
        def f(x, y, z): 
            result = x + y - z
            return result
    ";

    let lexer = Lexer::new(rules);
    let result = lexer.split(input);

    let tokens = result.ok().unwrap();
    assert_eq!(
        tokens.iter().map(|t| t.ty).collect::<Vec<_>>(),
        vec![
            "DEF",
            "IDENTIFIER",
            "LPAREN",
            "IDENTIFIER",
            "COMMA",
            "IDENTIFIER",
            "COMMA",
            "IDENTIFIER",
            "RPAREN",
            "COLON",
            "IDENTIFIER",
            "EQUAL",
            "IDENTIFIER",
            "PLUS",
            "IDENTIFIER",
            "MINUS",
            "IDENTIFIER",
            "RETURN",
            "IDENTIFIER"
        ]
    );
}

#[test]
pub fn lex_some_python_2() {
    let rules = vec![
        Rule::new("DEF", r"def".to_string()),
        Rule::new("RETURN", r"return".to_string()),
        Rule::new("LPAREN", r"\(".to_string()),
        Rule::new("RPAREN", r"\)".to_string()),
        Rule::new("COMMA", r",".to_string()),
        Rule::new("COLON", r":".to_string()),
        Rule::new("EQUAL", r"=".to_string()),
        Rule::new("PLUS", r"\+".to_string()),
        Rule::new("MINUS", r"\-".to_string()),
        Rule::new(
            "NUMBER",
            r"(\+|\-)?([0-9]+(\.[0-9]*)?|[0-9]*\.[0-9]+)".to_string(),
        ),
        Rule::new("IDENTIFIER", r"(_|[a-zA-Z])(_|[a-z0-9A-Z])*".to_string()),
        Rule::new_skip("WhiteSpace", r"(\s|\t|\n)+".to_string()),
    ];

    let input = r"
        def function_def(x, y, z): 
            result = x + y - z
            return result
    ";

    let lexer = Lexer::new(rules);
    let result = lexer.split(input);
    assert_eq!(
        result
            .ok()
            .unwrap()
            .iter()
            .map(|t| t.ty)
            .collect::<Vec<_>>(),
        vec![
            "DEF",
            "IDENTIFIER",
            "LPAREN",
            "IDENTIFIER",
            "COMMA",
            "IDENTIFIER",
            "COMMA",
            "IDENTIFIER",
            "RPAREN",
            "COLON",
            "IDENTIFIER",
            "EQUAL",
            "IDENTIFIER",
            "PLUS",
            "IDENTIFIER",
            "MINUS",
            "IDENTIFIER",
            "RETURN",
            "IDENTIFIER"
        ]
    );
}

#[test]
pub fn lex_some_c_sharp() {
    let rules = vec![
        Rule::new("PUBLIC", r"public".to_string()),
        Rule::new("STATIC", r"static".to_string()),
        Rule::new("CLASS", r"class".to_string()),
        Rule::new("RETURN", r"return".to_string()),
        Rule::new("LPAREN", r"\(".to_string()),
        Rule::new("RPAREN", r"\)".to_string()),
        Rule::new("LBRACKET", r"{".to_string()),
        Rule::new("RBRACKET", r"}".to_string()),
        Rule::new("COMMA", r",".to_string()),
        Rule::new("COLON", r":".to_string()),
        Rule::new("SEMICOLON", r";".to_string()),
        Rule::new("EQUAL", r"=".to_string()),
        Rule::new("LAMBDA", r"=>".to_string()),
        Rule::new("public", r"public".to_string()),
        Rule::new("PLUS", r"\+".to_string()),
        Rule::new("MINUS", r"\-".to_string()),
        Rule::new("STIRNG", r#""[^"-"]*""#.to_string()),
        Rule::new(
            "NUMBER",
            r"(\+|\-)?([0-9]+(\.[0-9]*)?|[0-9]*\.[0-9]+)".to_string(),
        ),
        Rule::new("IDENTIFIER", r"(_|[a-zA-Z])(_|[a-z0-9A-Z])*".to_string()),
        Rule::new_skip("WhiteSpace", r"(\s|\t|\n)+".to_string()),
    ];

    let input = "
        public static class Boniato: Hortaliza {
            public string Nombre() => \"boniato\";
        }
    ";

    let lexer = Lexer::new(rules);
    let result = lexer.split(input);

    let tokens = result.ok().unwrap();
    assert_eq!(
        tokens.iter().map(|t| t.ty).collect::<Vec<_>>(),
        vec![
            "PUBLIC",
            "STATIC",
            "CLASS",
            "IDENTIFIER",
            "COLON",
            "IDENTIFIER",
            "LBRACKET",
            "PUBLIC",
            "IDENTIFIER",
            "IDENTIFIER",
            "LPAREN",
            "RPAREN",
            "LAMBDA",
            "STIRNG",
            "SEMICOLON",
            "RBRACKET"
        ]
    );
}

#[test]
pub fn empty_match() {
    let rules = vec![Rule::new("EMPTY", "a*".to_string())];

    let program_text = "bbbbbb";

    let lexer = Lexer::new(rules);

    let mut error_handler = ErrorHandler::new(program_text);
    let errors = lexer.split(program_text).err().unwrap().1;

    error_handler.extend_errors(errors);

    assert_eq!(error_handler.errors.len(), 1);
    assert_eq!(
        error_handler.get_raw_errors(),
        vec!["Invalid character `b`",]
    );
}
