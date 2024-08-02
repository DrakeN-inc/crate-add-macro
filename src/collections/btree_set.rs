/// This macros provides the fast creating  [BTreeSet](std::collections::BTreeSet) object
/// 
/// # Examples:
/// ```
/// use add_macro::btree_set;
/// use std::collections::BTreeSet;
/// 
/// assert_eq!(
///     btree_set![1, 2, 3, 4],
///     {
///         let mut set = BTreeSet::new();
///         set.insert(1);
///         set.insert(2);
///         set.insert(3);
///         set.insert(4);
///         set
///     }
/// );
/// ```
#[macro_export]
macro_rules! btree_set {
    () => {
        BTreeSet::new()
    };

    ($($v:expr),+ $(,)?) => {{
        let mut set = std::collections::BTreeSet::new();
        $(set.insert($v);)*

        set
    }};
}
