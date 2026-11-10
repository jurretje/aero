#[derive(Debug)]
pub enum ElabError {
    UnknownIdentifier(String),
    CannotInferBinderType,
    CannotInferDeclarationType,
    UnsupportedExpression,
    TooManyBinders,
    DuplicateDeclaration(String),
}
