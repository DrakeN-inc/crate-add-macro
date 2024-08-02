/// This macros provides the fast creating  [Regex](https://docs.rs/regex) object
/// 
/// # Examples:
/// ```
/// use regex::Regex;
/// use add_macro::re;
/// 
/// let re: Regex = re!(r"^hello$");
/// 
/// assert!(re.is_match("hello"))
/// ```
#[macro_export]
macro_rules! re {
    ($s:expr) => {
        regex::Regex::new(&$s).unwrap()
    };
}
