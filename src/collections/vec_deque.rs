/// This macros provides the fast creating  [VecDeque](std::collections::VecDeque) object
/// 
/// # Examples:
/// ```
/// use add_macro::deq;
/// use std::collections::VecDeque;
/// 
/// assert_eq!(
///     deq![1, 2, 3, 4],
///     VecDeque::from([1, 2, 3, 4])
/// );
/// 
/// assert_eq!(
///     deq![0u8; 10].len(),
///     10
/// );
/// ```
#[macro_export]
macro_rules! deq {
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
