# Match Type

Match on the type of an expression at compile time `\(^O^)/`.

For example the following macro will expand to a different `String` expression depending on what the type of the input expression is:

```rs
struct A(i64,i32,i16,i8);
struct B(f64,f32,bool);

macro_rules! stringify {
    ($e:expr) => {
        match_type!(
            $e {
                A => String: "It's an A :O".to_string(),
                B => String: "B it is ^^".to_string(),
                <T: Display> T => String: format!("{}", self),
                <T: Debug> T => String: format!("{:?}", self),
                _ => String: "<<Sad Monkey :(>>".to_string(),
            }
        )
    }
}
```

Or this macro will "negate" a value, using a different operation depending on which is supported (`-` taking precedence):

```rs
macro_rules! inv {
    ($e:expr) => {
        match_type!(
            $e {
                <T: Neg> T => <T as Neg>::Output: -self,
                <T: Not> T => <T as Not>::Output: !self,
            }
        )
    };
}
```