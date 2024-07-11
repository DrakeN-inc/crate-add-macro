/// The macro to fast creating [String](std::string::String)
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
