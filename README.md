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

Note that the types of the different `match_type` arms do not have to be of the same type.

## How?

Obviously with trait trickery, what else. At the core of what makes this possible though is the following feature of Rust:

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

I.e. I can have the same method name multiple times, once directly implemented and once via a trait. If I write `X.f()`, then the `X::f` has precedence over the `Q::f`. Of course both `impl X {...}` and `impl Q for X {...}` can contain type & trait constraints, allowing us to control dependent on types which code actually gets executed for the expression `X.f()`.

In the implementation this is utilized as follows:

```rs
trait M0 {
    type R;
    fn m0(self) -> Self::R;
}

// For a type that matches the type pattern, the trait `M0` is implemented.
impl M0 for /* type to be matched */ {
    type R = /* return type of match arm */;

    fn m0(self) -> Self::R {
        /* right hand side of match arm */
    }
}

struct W0<T>(T);

// For a type for which M0 is implemented, `W0::w0` is implemented.
impl<T: M0> W0<T> {
    fn w0(self) -> Done<<T as M0>::R> {
        Done(M0::m0(self.0))
    }
}

trait C0 {
    type R;
    fn w0(self) -> Self::R;
}

// For all types `C0::w0` is implemented.
impl<T> C0 for W0<T> {
    type R = W1<T>;
    fn w0(self) -> Self::R {
        W1(self.0)
    }
}
```

The same with `M1`, `W1` and `C1` and so on, for every arm in our `match_type`.

Consequently, for any value `x` that matches the type pattern, `x.w0()` evaluates to whatever is the right hand side of match arm. For any value `x` that does not match the type pattern, `x.w0()` evaluates to `W1(x)`.

The `Done` Type is simply a wrapper for which we also implement `w0()`, `w1()`, `w2()` and so on, all as `id`.

So in the end, for a `match_type` with say 4 arms, we have an expression like:

```rs
W0(x).w0().w1().w2().w3().done()
```

Depending on the type of `x`, one of the `w*()` method calls will return a `Done(..)` value, which then gets handed through until the end, where it is unwrapped by `done()` and we get the desired value. For example, if `x` matches the third (index `2`) type pattern, then `.w0` will be `C0::w0`, `w1` will be `C1::w1`, `w2` will be `W2::w2` and `w3` will be `Done::w3`:

```rs
let x: i32 = 1729;
let y: String = W0(x) // W0<i32>
.w0()                 // W1<i32>
.w1()                 // W2<i32>
.w2()                 // Done<String>
.w3()                 // Done<String>
.done()               // String
```