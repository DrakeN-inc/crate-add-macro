# Crate Rust "add_macro" 

[![github]](https://github.com/DrakeN-inc/crate-add-macro)&ensp;[![crates-io]](https://crates.io/crates/add_macro)&ensp;[![docs-rs]](https://docs.rs/add_macro)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

## This crate provides the more additional macros to help you write code faster!

# Examples:
```rust
use add_macro::{ re, Display, From, FromStr };

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Display, From)]
enum Error {
    #[from]
    Io(std::io::Error),
    
    #[display = "Incorrect E-mail address format"]
    IncorrectEmail,
}

#[derive(Debug, Display, FromStr)]
#[display = "{value}"]
struct Email {
    value: String,
}

impl Email {
    fn parse(s: &str) -> Result<Self> {
        let re = re!(r"^[\w\-]+@[\w\-]+\.\w+$");

        if re.is_match(s) {
            Ok(Self {
                value: s.into(),
            })
        } else {
            Err(Error::IncorrectEmail)
        }
    }
}

#[derive(Debug, Display)]
#[display = "Name: {name}, Age: {age}, E-mail: {email}"]
struct Person {
    name: String,
    age: u8,
    email: Email,
}

impl Person {
    pub fn new<S>(name: S, age: u8, email: Email) -> Self
    where S: Into<String> {
        Self {
            name: name.into(),
            age,
            email,
        }
    }
}

fn main() -> Result<()> {
    let bob = Person::new("Bob", 22, "bob@example.loc".parse()?);

    println!("{bob}");

    Ok(())
}
```
