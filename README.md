# Match Type

‖ [__Docs.rs__](https://docs.rs/match_type/latest/match_type/) ‖ [__Lib.rs__](https://lib.rs/crates/match_type) ‖ [__Crates.io__](https://crates.io/crates/match_type/) ‖

Match on the type of an expression at compile time.

For example the following macro will expand to a different `String` expression depending on what the type of the input expression is:

```rust
struct A(i64,i32,i16,i8);
struct B(f64,f32,bool);

macro_rules! as_string {
    ($e:expr) => {
        match_type!(
            $e {
                A => String: "It's an A :O".into(),
                B => String: "B it is ^^".into(),
                <T: Display> T => String: format!("{}", self),
                <T: Debug> T => String: format!("{:?}", self),
                _ => String: "<<Sad Monkey :(>>".into(),
            }
        )
    }
}
```

Or this macro will give us the "opposite" of a value, using a different operation depending on which is supported (with `-` taking precedence in case both `!` and `-` are):

```rust
macro_rules! opposite {
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

Note that the right hand sides of the  different match arms do not have to be of the same type.

## Performance

With `opt-level` of at least `1`, e.g. in the default `release` profile, the whole expansion of `match_type` should be inlined away to the point that the assembly output is the same as if you had written the right hand side of the matching arm directly. I.e. `match_type` should have zero production runtime overhead.

## How?

See [HOW.md](HOW.md)