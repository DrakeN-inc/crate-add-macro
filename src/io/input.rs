/// This macros provides the usefull reading buffer by using [io::stdin](std::io::stdin)
/// 
/// # Examples:
/// ```
/// use add_macro::input;
/// 
/// let buf: String = input!("Type something: ");
/// println!("{buf}");
/// ```
#[macro_export]
macro_rules! input {
    () => {{
        let mut buf = String::new();

        if let Ok(_) = std::io::stdin().read_line(&mut buf) {
            buf.trim().to_owned()
        } else {
            String::new()
        }
    }};

    ($($arg:tt)*) => {{
        eprint!($($arg)*);
        input!()
    }};
}
