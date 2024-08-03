use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ImplementationError,
    IncorrectAttribute,
    IncorrectAttributeVariant,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::ImplementationError => write!(f, "Implementation of trait 'Display' can be derived for Struct's and Enum's only!"),
            Self::IncorrectAttribute => write!(f, "Incorrect attribute value, correct formats is: #[from(\"type..\" = \"expression..\")] and for enum variants: #[from] or #[from = \"expression..\"] or #[from(\"type..\")] or #[from(\"type..\" = \"expression..\")]"),
            Self::IncorrectAttributeVariant => write!(f, "Cannot to use attributes #[from] or #[from = \"expr..\"] to enum variant without fields"),
        }
    }
}
