/// The macro to fast creating [VecDeque](std::collections::VecDeque)
/// 
/// # Examples:
/// ```
/// use add_macro::vec_deque;
/// use std::collections::VecDeque;
/// 
/// assert_eq!(
///     vec_deque![1, 2, 3, 4],
///     VecDeque::from([1, 2, 3, 4])
/// );
/// 
/// assert_eq!(
///     vec_deque![0u8; 10].len(),
///     10
/// );
/// ```
#[macro_export]
macro_rules! vec_deque {
    () => {
        VecDeque::new()
    };

    ($v:expr; $n:expr) => {{
        let mut vec = VecDeque::with_capacity($n);
        for _ in 0..$n {
            vec.push_back($v);
        }

        vec
    }};
    
    ($($v:expr),+ $(,)?) => {
        std::collections::VecDeque::from([$($v,)*])
    };
}
