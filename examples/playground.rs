use match_type::match_type;
use std::fmt;

fn main() {
    // let x = vec![1, 2, 3];
    let x = 1729;

    let y = match_type! {
        x {
            <T: fmt::Display> T => String: format!("{}", self),
            _ => bool: false,
        }
    };

    println!("{:#?}", y);
}
