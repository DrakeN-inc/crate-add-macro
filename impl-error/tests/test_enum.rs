extern crate add_macro_impl_error;
use add_macro_impl_error::Error;

#[derive(Debug, Error)]
enum Error {
    Io(std::io::Error),
    Wrong,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Io(e) => write!(f, "{e}"),
            Self::Wrong => write!(f, "Something went wrong.. =/"),
        }
    }
}

#[test]
fn test_enum() {
    let err = Error::Wrong;
    assert_eq!(format!("{err}"), "Something went wrong.. =/");
}
