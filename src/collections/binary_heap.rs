/// This macros provides the fast creating  [BinaryHeap](std::collections::BinaryHeap) object
/// 
/// # Examples:
/// ```
/// use std::collections::BinaryHeap;
/// use add_macro::heap;
/// 
/// fn main() {
///     assert_eq!(
///         heap![&1, &2, &3, &4].pop(),
///         BinaryHeap::from([&1, &2, &3, &4]).pop()
///     );
/// 
///     assert_eq!(
///         heap![&1; 10].len(),
///         10
///     );
/// }
/// ```
#[macro_export]
macro_rules! heap {
    () => {
        BinaryHeap::new()
    };

    ($v:expr; $n:expr) => {{
        let mut vec = BinaryHeap::with_capacity($n);
        for _ in 0..$n {
            vec.push($v);
        }

        vec
    }};
    
    ($($v:expr),+ $(,)?) => {
        std::collections::BinaryHeap::from([$($v,)*])
    };
}
