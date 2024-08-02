/// This macros provides the fast creating  [String](std::string::String) object
/// 
/// # Examples:
/// ```
/// use add_macro::str;
/// 
/// let empty_s: String = str!();
/// let s: String = str!("Hello, world!");
/// ```
#[macro_export]
macro_rules! str {
    () => {
        String::new()
    };
    ($s:expr) => {
        $s.to_owned()
    };
}
