pub enum LexicalError {
    InvalidCharacter(String),
    UnclosedStringLiteral(String),
    UnclosedComment(String),
}
