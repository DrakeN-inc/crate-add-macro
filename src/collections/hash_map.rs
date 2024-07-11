/// The macro to fast creating [HashMap](std::collections::HashMap)
/// 
/// # Examples:
/// ```
/// use add_macro::hash_map;
/// use std::collections::HashMap;
/// 
/// assert_eq!(
///     hash_map! {
///         "key1" => "value1",
///         "key2" => "value2",
///     },
///     HashMap::from([("key1", "value1"), ("key2", "value2")])
/// );
/// ```
#[macro_export]
macro_rules! hash_map {
    () => {
        std::collections::HashMap::new()
    };
    
    ($($k:expr => $v:expr),+ $(,)?) => {
        std::collections::HashMap::from([$(($k, $v),)*])
    };
}
