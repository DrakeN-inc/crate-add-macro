/// This macros provides the fast creating  [HashSet](std::collections::HashSet) object
/// 
/// # Examples:
/// ```
/// use add_macro::set;
/// use std::collections::HashSet;
/// 
/// assert_eq!(
///     set![1, 2, 3, 4],
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
macro_rules! set {
    () => {
        HashSet::new()
    };
    
    ($($v:expr),+ $(,)?) => {{
        let mut set = std::collections::HashSet::new();
        $(set.insert($v);)*

        set
    }};
}
