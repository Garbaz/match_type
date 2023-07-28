# Match Type

Expand macro differently depending on type of input expression.

For example:

```rs
use match_type::match_type;

macro_rules! m {
    ($e:expr) => {
        match_type!{
            $e {
                <T> Vec<T> => usize: self.len(),
                usize => f64: self as f64,
                f64 => bool: self.is_finite(),

            }
        }
    };
}
```

or:

```rs
use match_type::match_type;
use std::fmt::Display;

fn f<T>(x : T) -> String {
    match_type!{
        x {
            <D: Display> D => String: format!("{}", self),
            _ => String: "No Display :(".to_string(),
        }
    }
}
```

# TODO

- In the above `f` example, if the inner type variable is also named `T` then it doesn't work. Somehow the outer type variable is leaking in ._. That shouldn't happen.
- Allow for precedence ordering such that we can fall down to a lower `<D: std::fmt::Debug> D` case in the `f` example. This should be possible with the same trick as we do the default case with (?).