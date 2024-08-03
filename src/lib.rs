//! This crate adds the some useful macros for easily work and save your time

pub mod string;
pub mod collections;
pub mod io;


/* TODO:
    use add_macro_impl_display::Display;
    use add_macro_impl_error::Error;
    use add_macro_impl_fromstr::FromStr;
    use add_macro_impl_from::From;
    use add_macro_impl_into::Into;

    #[derive(Debug, Display, Error, From)]
    #[from(std::io::Error = "Self::Io(v)")]
    enum Error {
        #[display("{}")]
        Io(std::io::Error),

        #[display("{}")]
        Regex(#[from] regex::Error),

        #[display("Wrong a login or password data!")]
        WrongLoginData,
    }

    fn main() {
        let err = Error::WrongLoginData;

        assert_eq!(err.source(), "Wrong a login or password data!");
    }
*/
