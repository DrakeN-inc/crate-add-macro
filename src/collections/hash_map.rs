/// This macros provides the fast creating  [HashMap](std::collections::HashMap) object
/// 
/// # Examples:
/// ```
/// use add_macro::map;
/// use std::collections::HashMap;
/// 
/// assert_eq!(
///     map! {
///         "key1" => "value1",
///         "key2" => "value2",
///     },
///     HashMap::from([("key1", "value1"), ("key2", "value2")])
/// );
/// ```
#[macro_export]
macro_rules! map {
    () => {
        std::collections::HashMap::new()
    };
    
    ($($k:expr => $v:expr),+ $(,)?) => {
        std::collections::HashMap::from([$(($k, $v),)*])
    };
}
