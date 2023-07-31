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
