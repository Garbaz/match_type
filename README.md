# Match Type

‖ [__Docs.rs__](https://docs.rs/match_type/latest/pronto_graphics/) ‖ [__Lib.rs__](https://lib.rs/crates/match_type) ‖ [__Crates.io__](https://crates.io/crates/match_type/) ‖

Match on the type of an expression at compile time.

For example the following macro will expand to a different `String` expression depending on what the type of the input expression is:

```rs
struct A(i64,i32,i16,i8);
struct B(f64,f32,bool);

macro_rules! stringify {
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

```rs
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

With `opt-level` of at least `1`, e.g. in the default `release` profile, the whole expansion of `match_type` should be inlined away to the point that the assembly output is the same as if you had written the right hand side of the matching arm directly. I.e. `match_type` has zero production runtime overhead.

## How?

Obviously with trait trickery, what else. At the core of what makes this possible is the following feature of Rust:

```rs
struct X;
impl X {
    fn f(self) {}
}

trait Q {
    fn f(self) {}
}
impl Q for X {}
```

I.e. I can have for one type the same method name twice, once directly implemented and once via a trait. If that is the case, the directly implemented function shadows the trait implemented function. Of course both `impl X {...}` and `impl Q for X {...}` can contain type & trait constraints, allowing us to control dependent on types which of the two `f` actually gets executed for the expression `x.f()` for some value `x: X`.

In the implementation this is utilized as follows:

```rs
struct Done<T>(T);

impl<T> Done<T> {
    fn arm0(self) -> Self {
        self
    }
    fn arm1(self) -> Self {
        self
    }
    fn done(self) -> T {
        self.0
    }
}

struct Wrapper<T>(T);

trait Catch<T> {
    fn arm0(self) -> Self;
    fn arm1(self) -> Self;
}

impl<T> Catch<T> for Wrapper<T> {
    fn arm0(self) -> Self {
        self
    }
    fn arm1(self) -> Self {
        self
    }
}

trait Match0 {
    type ReturnType = /* arm 0 RHS expression type */;
    fn match(self) -> Self::ReturnType;
}

impl Match0 for /* arm 0 match type */ {
    fn match(self) -> Self::ReturnType {
        /* arm 0 RHS expression */
    }
}

impl<T: Match0> Wrapper<T> {
    fn arm0(self) -> Done<<T as Match0>::ReturnType> {
        Done(Match0::match(self.0))
    }
}

trait Match1 {
    type ReturnType = /* arm 1 RHS expression type */;
    fn match(self) -> Self::ReturnType;
}

impl Match1 for /* arm 1 match type */ {
    fn match(self) -> Self::ReturnType {
        /* arm 1 RHS expression */
    }
}

impl<T: Match1> Wrapper<T> {
    fn arm1(self) -> Done<<T as Match1>::ReturnType> {
        Done(Match1::match(self.1))
    }
}
```

And so on with `arm{N}` & `Match{N}` for as many arms the `match_type` expression has.

Then the actual expression to be evaluated is:

```rs
Wrapper(x).arm0().arm1().done();
```

If `x` matches the type of the 0th match arm, the `arm0` in the expression will be `Wrapper::arm0`, which in turn executes the RHS of the match arm. The `arm1` in the expression then will be `Done::arm1`, and finally `done()` will unwrap the result.

If `x` does not match the type of the 0th match arm, the `arm0` in the expression will be `Catch::arm0`. If `x` does match the type of the 1st match arm, then `arm1` will be `Wrapper::arm1`, and `done` again will unwrap the result.

If `x` matches neither the 0th match arm, nor the 1st match arm, we get a type error (at compile time!), since `done` is not implemented for Wrapper.
