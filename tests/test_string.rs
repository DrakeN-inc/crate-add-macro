extern crate add_macro;
#[allow(unused_imports)]
use add_macro::{ str, re };
#[cfg(feature = "tests")]
use regex::Regex;

// test string:
#[test]
fn test_str() {
    let _empty_s: String = str!();
    let _s: String = str!("Hello, world!");
}


// test regex:
#[test]
#[cfg(feature = "tests")]
fn test_regex() {
    let re: Regex = re!(r"^hello$");
    
    assert!(re.is_match("hello"))
}
