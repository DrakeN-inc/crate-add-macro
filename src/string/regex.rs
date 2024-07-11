/// The macro to fast creating [Regex](https://docs.rs/regex)
/// 
/// # Examples:
/// ```
/// use add_macro::regex;
/// use regex::Regex;
/// 
/// let re: Regex = regex!(r"^hello$");
/// ```
#[macro_export]
macro_rules! regex {
    ($s:expr) => {
        regex::Regex::new(&$s).unwrap()
    };
}
