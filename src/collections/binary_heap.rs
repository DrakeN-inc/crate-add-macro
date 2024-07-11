/// The macro to fast creating [BinaryHeap](std::collections::BinaryHeap)
/// 
/// # Examples:
/// ```
/// use add_macro::binary_heap;
/// use std::collections::BinaryHeap;
/// 
/// assert_eq!(
///     binary_heap![&1, &2, &3, &4].pop(),
///     BinaryHeap::from([&1, &2, &3, &4]).pop()
/// );
/// 
/// assert_eq!(
///     binary_heap![&1; 10].len(),
///     10
/// );
/// 
/// ```
#[macro_export]
macro_rules! binary_heap {
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
