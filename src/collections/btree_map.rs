/// The macro to fast creating [BTreeMap](std::collections::BTreeMap)
/// 
/// # Examples:
/// ```
/// use add_macro::btree_map;
/// use std::collections::BTreeMap;
/// 
/// assert_eq!(
///     btree_map! {
///         "key1" => "value1",
///         "key2" => "value2",
///     },
///     BTreeMap::from([("key1", "value1"), ("key2", "value2")])
/// );
/// ```
#[macro_export]
macro_rules! btree_map {
    () => {
        std::collections::BTreeMap::new()
    };
    
    ($($k:expr => $v:expr),+ $(,)?) => {
        std::collections::BTreeMap::from([$(($k, $v),)*])
    };
}
