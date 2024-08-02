extern crate add_macro;
use add_macro::input;

fn main() {
    let buf = input!("Type something: ");
    println!("{buf}");
}
