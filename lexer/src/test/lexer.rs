use std::vec;

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

    assert!(result.is_ok());
    let tokens = result.unwrap();
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

    assert!(result.is_ok());
    let tokens = result.unwrap();
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

    assert!(result.is_ok());
    let tokens = result.unwrap();
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

    assert!(result.is_ok());
    let tokens = result.unwrap();
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

    assert!(result.is_ok());
    let tokens = result.unwrap();
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

    let lexer = Lexer::new(rules);

    let result = lexer.split("aab bba ccc").err().unwrap();

    assert_eq!(result.len(), 3);
    assert_eq!(
        result,
        vec![
            "Lexical Error: Unexpected character 'c' at line: 0, column: 8",
            "Lexical Error: Unexpected character 'c' at line: 0, column: 9",
            "Lexical Error: Unexpected character 'c' at line: 0, column: 10"
        ]
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

    let result = lexer.split("aab bba caac");

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 2);
    assert_eq!(
        errors,
        vec![
            "Lexical Error: Unexpected character 'c' at line: 0, column: 8",
            "Lexical Error: Unexpected character 'c' at line: 0, column: 11",
        ]
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

    assert!(result.is_ok());
    let tokens = result.unwrap();
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
    let result = lexer.split("a\nb\nc");
    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 2);
    assert_eq!(
        errors,
        vec![
            "Lexical Error: Unexpected character 'b' at line: 2, column: 1",
            "Lexical Error: Unexpected character 'c' at line: 3, column: 1"
        ]
    );
}
