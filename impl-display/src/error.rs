use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ImplementationError,
    IncorrectAttribute,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::ImplementationError => write!(f, "Implementation of trait 'Display' can be derived for Struct's and Enum's only!"),
            Self::IncorrectAttribute => write!(f, "Incorrect attribute value, correct formats is: #[display] or #[display(\"...\")] or #[display = \"...\"]"),
        }
    }
}
