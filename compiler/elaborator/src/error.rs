use surface::QualifiedIdent;

#[derive(Debug)]
pub enum ElabError {
    UnknownIdentifier(QualifiedIdent),
    CannotInferBinderType,
    CannotInferDeclarationType,
    UnsupportedExpression,
    TooManyBinders,
}
