/// The macro to fast creating [LinkedList](std::collections::LinkedList)
/// 
/// # Examples:
/// ```
/// use add_macro::linked_list;
/// use std::collections::LinkedList;
/// 
/// assert_eq!(
///     linked_list![1, 2, 3, 4],
///     LinkedList::from([1, 2, 3, 4])
/// );
/// 
/// assert_eq!(
///     linked_list![0u8; 10].len(),
///     10
/// );
/// ```
#[macro_export]
macro_rules! linked_list {
    () => {
        LinkedList::new()
    };

    ($v:expr; $n:expr) => {{
        let mut list = LinkedList::new();
        for _ in 0..$n {
            list.push_back($v);
        }

        list
    }};
    
    ($($v:expr),+ $(,)?) => {
        std::collections::LinkedList::from([$($v,)*])
    };
}
