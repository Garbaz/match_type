use match_type::match_type;
use std::fmt::{Debug, Display};

macro_rules! as_string {
    ($e:expr) => {
        match_type!(
            $e {
                <T: Display> T => String: format!("{}", self),
                <T: Debug>   T => String: format!("{:?}", self),
                _              => String: stringify!($e).into(),
            }
        )
    };
}

struct Foo(u32);

fn f<T>(x: T) -> String {
    as_string!(x)
}

fn main() {
    assert_eq!("Hello World", as_string!("Hello World"));
    assert_eq!("1729", as_string!(1729));
    assert_eq!("[1, 2, 3]", as_string!(vec![1, 2, 3]));
    assert_eq!(
        "Foo(9 * 9 * 9 + 10 * 10 * 10)",
        as_string!(Foo(9 * 9 * 9 + 10 * 10 * 10))
    );
    assert_eq!("x", f("Sad Monkey")); // (!)
}
