/// The macro to fast creating [HashSet](std::collections::HashSet)
/// 
/// # Examples:
/// ```
/// use add_macro::hash_set;
/// use std::collections::HashSet;
/// 
/// assert_eq!(
///     hash_set![1, 2, 3, 4],
///     {
///         let mut set = HashSet::new();
///         set.insert(1);
///         set.insert(2);
///         set.insert(3);
///         set.insert(4);
///         set
///     }
/// );
/// ```
#[macro_export]
macro_rules! hash_set {
    () => {
        HashSet::new()
    };
    
    ($($v:expr),+ $(,)?) => {{
        let mut set = std::collections::HashSet::new();
        $(set.insert($v);)*

        set
    }};
}
