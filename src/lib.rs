//! This crate provides the more additional macros to help you write code faster!

pub mod string;
pub mod collections;
pub mod io;


/// This macros provides the implementation of trait [Display](std::fmt::Display)
/// 
/// # Examples:
/// ```
/// use add_macro_impl_display::Display;
/// 
/// #[derive(Display)]
/// enum Animals {
///     Turtle,
///   
///     #[display]
///     Bird,
///    
///     #[display = "The Cat"]
///     Cat,
/// 
///     #[display("The Dog")]
///     Dog,
///
///     Other(&'static str),
///
///     #[display]
///     Other2(&'static str),
/// 
///     #[display("{0} {1}, {2} years old")]
///     Info(Box<Self>, &'static str, i32),
///
///     #[display("{kind} {name}, {age} years old")] 
///     Info2 {
///         kind: Box<Self>,
///         name: &'static str,
///         age: i32,
///     },
/// }
///
/// fn main() {
///     assert_eq!(format!("{}", Animals::Turtle), "Turtle");
///     assert_eq!(format!("{}", Animals::Bird), "Bird");
///     assert_eq!(format!("{}", Animals::Cat), "The Cat");
///     assert_eq!(format!("{}", Animals::Dog), "The Dog");
///     assert_eq!(format!("{}", Animals::Other("Tiger")), "Tiger");
///     assert_eq!(format!("{}", Animals::Other2("Tiger")), "Tiger");
///     assert_eq!(format!("{}", Animals::Info(Box::new(Animals::Cat), "Tomas", 2)), "The Cat Tomas, 2 years old");
///     assert_eq!(format!("{}", 
///         Animals::Info2 {
///             kind: Box::new(Animals::Cat),
///             name: "Tomas",
///             age: 2,
///         }),
///         "The Cat Tomas, 2 years old"
///     );
/// }
/// ```
/// ```
/// use add_macro_impl_display::Display;
/// 
/// #[derive(Debug, Display)]
/// struct Person {
///     pub name: String,
///     pub age: u8,
/// }
/// 
/// #[derive(Debug, Display)]
/// #[display("Hello, {name}! Your age is {age} years old.")]
/// struct Person2 {
///     pub name: String,
///     pub age: u8,
/// }
///
/// fn main() {
///     assert_eq!(
///         format!("{}", Person {
///             name: "Bob".to_owned(),
///             age: 22
///         }),
///         "Person { name: \"Bob\", age: 22 }"
///     );
/// 
///     assert_eq!(
///         format!("{}", Person2 {
///             name: "Bob".to_owned(),
///             age: 22
///         }),
///         "Hello, Bob! Your age is 22 years old."
///     );
/// }
/// ```
pub use add_macro_impl_display::Display;


/// The implementation of trait 'Error'
/// 
/// # Examples:
/// ```
/// use add_macro_impl_error::Error;
/// 
/// #[derive(Debug, Error)]
/// enum CustomError {
///     Io(std::io::Error),
///     Wrong,
/// }
///
/// impl std::fmt::Display for CustomError {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         match &self {
///             Self::Io(e) => write!(f, "{e}"),
///             Self::Wrong => write!(f, "Something went wrong.. =/"),
///         }
///     }
/// }
///
/// #[derive(Debug, Error)]
/// pub struct Error {
///     source: CustomError,
/// }
///
/// impl std::fmt::Display for Error {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         write!(f, "{}", &self.source)
///     }
/// }
///
/// fn main() {
///     let err = CustomError::Wrong;
///     assert_eq!(format!("{err}"), "Something went wrong.. =/");
/// 
///     let err = Error { source: CustomError::Wrong };
///     assert_eq!(format!("{err}"), "Something went wrong.. =/");
/// }
/// ```
pub use add_macro_impl_error::Error;


/// This macros provides the implementation of the trait [FromStr](std::str::FromStr)
/// 
/// # Examples:
/// ```
/// use add_macro_impl_fromstr::FromStr;
/// 
/// #[derive(Debug)]
/// enum Error {
///     ParseError,
/// }
/// 
/// #[derive(FromStr)]
/// struct Email {
///     name: String,
///     host: String
/// }
/// 
/// impl Email {
///     // WARNING: this method needs for working the implementation trait FromStr
///     pub fn parse(s: &str) -> Result<Self, Error> {
///         let spl = s.trim().splitn(2, "@").collect::<Vec<_>>();
///         
///         if spl.len() == 2 {
///             Ok(Self {
///                 name: spl[0].to_owned(),
///                 host: spl[1].to_owned(),
///             })
///         } else {
///             Err(Error::ParseError)
///         }
///     }
/// }
/// 
/// fn main() -> Result<(), Error> {
///     let _bob_email: Email = "bob@example.loc".parse()?;
/// 
///     Ok(())
/// }
/// ```
pub use add_macro_impl_fromstr::FromStr;


/// This macros provides the implementation of trait [From<T>](std::convert::From)
/// 
/// # Examples:
/// ```
/// use add_macro_impl_from::From;
/// 
/// #[derive(Debug)]
/// enum SimpleError {
///     Wrong,
/// }
///
/// impl std::fmt::Display for SimpleError {
///     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
///         match &self {
///             Self::Wrong => write!(f, "Something went wrong.. =/"),
///         }
///     }
/// }
///
/// #[derive(Debug)]
/// struct SuperError {
///     source: String,
/// }
///
/// impl std::fmt::Display for SuperError {
///     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
///         write!(f, "{}", &self.source)
///     }
/// }
///
/// #[derive(Debug, From)]
/// #[from("std::io::Error" = "Self::Io(v)")]       // result: impl From<std::io::Error> for Error { fn from(v: std::io::Error) -> Self { Self::Io(v) } }
/// enum Error {
///     Io(std::io::Error),
///
///     #[from]
///     Simple(SimpleError),
///
///     #[from = "SuperError { source: format!(\"Super error: {}\", v.source) }"]
///     Super(SuperError),
///
///     #[from("String")]
///     #[from("&str" = "v.to_owned()")]
///     #[from("i32" = "format!(\"Error code: {v}\")")]
///     Stringify(String),
/// }
///
/// impl std::fmt::Display for Error {
///     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
///         match &self {
///             Self::Io(e) => write!(f, "{e}"),
///             Self::Simple(e) => write!(f, "{e}"),
///             Self::Super(e) => write!(f, "{e}"),
///             Self::Stringify(e) => write!(f, "{e}"),
///         }
///     }
/// }
///
/// fn main() {
///     let io_err = Error::from( std::fs::read("fake/path/to/file").unwrap_err() );
///     assert_eq!(format!("{io_err}"), "No such file or directory (os error 2)");
///
///     let simple_err = Error::from( SimpleError::Wrong );
///     assert_eq!(format!("{simple_err}"), "Something went wrong.. =/");
///
///     let super_err = Error::from( SuperError { source: "Bad request".to_owned() } );
///     assert_eq!(format!("{super_err}"), "Super error: Bad request");
///
///     let str_err = Error::from( String::from("Something went wrong.. =/") );
///     assert_eq!(format!("{str_err}"), "Something went wrong.. =/");
///
///     let str_err2 = Error::from("Something went wrong.. =/");
///     assert_eq!(format!("{str_err2}"), "Something went wrong.. =/");
///
///     let str_err3 = Error::from(404);
///     assert_eq!(format!("{str_err3}"), "Error code: 404");
/// }
/// ```
pub use add_macro_impl_from::From;


/// This macros provides the implementation of trait [Into<T>](std::convert::Into)
/// 
/// # Examples:
/// ```
/// use add_macro_impl_into::Into;
///
/// #[derive(Debug, PartialEq)]
/// struct User {
///     name: String,
///     subname: Option<String>,
///     age: u8
/// }
///
/// #[derive(Debug, Clone, Into)]
/// #[into("User" = "User { name: self.name, subname: None, age: self.age }")]
/// #[into("String" = "format!(\"name: {}, age: {}\", self.name, self.age)")]
/// struct Person {
///     name: String,
///     age: u8
/// }
///
/// fn main() {
///     let bob = Person {
///         name: "Bob".to_owned(),
///         age: 22
///     };
///
///     let bob_user: User = bob.clone().into();
///     assert_eq!(
///         bob_user,
///         User {
///             name: "Bob".to_owned(),
///             subname: None,
///             age: 22
///         }
///     );
///
///     let bob_str: String = bob.into();
///     assert_eq!(bob_str, "name: Bob, age: 22");
/// }
/// ```
/// ```
/// use add_macro_impl_into::Into;
///
/// #[derive(Debug, PartialEq)]
/// struct Cat;
///
/// #[derive(Debug, PartialEq)]
/// struct Dog;
///
/// #[derive(Debug, PartialEq)]
/// struct Bird;
///
/// #[derive(Debug, PartialEq)]
/// struct Python;
///
/// #[derive(Debug, PartialEq, Into)]
/// #[into("String" = "format!(\"Animal::{self:?}\")")]
/// #[into("Option<Cat>" = "if let Self::Cat(v) = self { Some(v) }else{ None }")]
/// enum Animal {
///     Cat(Cat),
///
///     #[into]
///     Dog(Dog),   // Option<Dog>
/// 
///     #[into = "if let Self::Bird(value) = self { value }else{ panic!(\"It's not a dog.\") }"]
///     // #[into("if let Self::Bird(value) = self { value }else{ panic!(\"It's not a dog.\") }")]
///     Bird(Bird),
/// 
///     #[into("Option<Python>" = "if let Self::Python(v) = self { Some(v) }else{ None }")]
///     #[into("Python" = "Into::<Option<Python>>::into(self).expect(\"It's not a Python\")")]
///     Python(Python),
/// }
///
/// fn main() {
///     let cat_str: String = Animal::Cat(Cat {}).into();
///     assert_eq!(cat_str, "Animal::Cat(Cat)");
/// 
///     let cat: Option<Cat> = Animal::Cat(Cat {}).into();
///     assert_eq!(cat, Some(Cat {}));
/// 
///     let dog: Option<Dog> = Animal::Dog(Dog {}).into();
///     assert_eq!(dog, Some(Dog {}));
///
///     let bird: Bird = Animal::Bird(Bird {}).into();
///     assert_eq!(bird, Bird {});
///
///     let python: Option<Python> = Animal::Python(Python {}).into();
///     assert_eq!(python, Some(Python {}));
///
///     let python: Python = Animal::Python(Python {}).into();
///     assert_eq!(python, Python {});
/// }
/// ```
pub use add_macro_impl_into::Into;
