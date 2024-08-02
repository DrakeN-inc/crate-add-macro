extern crate add_macro_impl_fromstr;
use add_macro_impl_fromstr::FromStr;

#[derive(Debug)]
enum Error {
    ParseError,
}

#[derive(FromStr)]
struct Email {
    name: String,
    host: String
}

impl Email {
    // WARNING: this method needs for the implement trait FromStr
    pub fn parse(s: &str) -> Result<Self, Error> {
        let spl = s.trim().splitn(2, "@").collect::<Vec<_>>();
        
        if spl.len() == 2 {
            Ok(Self {
                name: spl[0].to_owned(),
                host: spl[1].to_owned(),
            })
        } else {
            Err(Error::ParseError)
        }
    }
}

#[test]
fn test_fromstr() -> Result<(), Error> {
    let _bob_email: Email = "bob@example.loc".parse()?;

    Ok(())
}
