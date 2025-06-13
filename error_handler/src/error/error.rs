pub enum HulkError {
    LexicalError(String),
    SyntacticError(String),
    SemanticError(String),
}
