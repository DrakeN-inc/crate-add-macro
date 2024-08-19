extern crate add_macro_impl_from;
use add_macro_impl_from::From;

#[derive(Debug, From)]
#[from("String" = "Self { source: v }")]
#[from("&str" = "Self { source: v.to_owned() }")]
#[from("i32" = "Self { source: format!(\"Error code: {v}\") }")]
struct Error {
    source: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.source)
    }
}

#[test]
fn test_struct() {
    let _err = Error::from( String::from("Something went wrong.. =/") );

    let err2 = Error::from("Something went wrong.. =/");
    assert_eq!(format!("{err2}"), "Something went wrong.. =/");

    let err3 = Error::from(404);
    assert_eq!(format!("{err3}"), "Error code: 404");
}
