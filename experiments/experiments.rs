trait __MatchType {
    type ReturnType;
    fn __match_type_arm(self) -> Self::ReturnType;
}

struct __MatchTypeWrapper<T>(T);

impl<T: __MatchType> __MatchTypeWrapper<T> {
    fn __match_type_arm(self) -> <T as __MatchType>::ReturnType {
        self.0.__match_type_arm()
    }
}

type __MatchTypeDefaultReturnType = usize;

trait __MatchTypeDefault {
    fn __match_type_arm(self) -> __MatchTypeDefaultReturnType;
}

impl<T> __MatchTypeDefault for T {
    fn __match_type_arm(self) -> __MatchTypeDefaultReturnType {
        1729
    }
}

impl __MatchType for i8 {
    type ReturnType = bool;
    fn __match_type_arm(self) -> Self::ReturnType {
        false
    }
}

impl __MatchType for bool {
    type ReturnType = i8;

    fn __match_type_arm(self) -> Self::ReturnType {
        -3
    }
}

impl<T> __MatchType for Vec<T> {
    type ReturnType = u16;

    fn __match_type_arm(self) -> Self::ReturnType {
        12
    }
}

fn main() {
    let _x = __MatchTypeWrapper(false).__match_type_arm();
    let _y = __MatchTypeWrapper(-3i8).__match_type_arm();
    let _z = __MatchTypeWrapper("hello").__match_type_arm();
    let _w = __MatchTypeWrapper(vec![1, 2, 3]).__match_type_arm();

    let c = vec![1, 2, 3, 4, 5];

    let _v = {
        trait Match {
            type R;
            fn q(self) -> Self::R;
        }

        impl<T> Match for Vec<T> {
            type R = usize;

            fn q(self) -> Self::R {
                // c.len()
                // ^ How can we make this word?
                self.len()
            }
        }

        let r = (vec![1, 2, 3]).q();
        r
    };
}

/*
let f = |s| c.len();
    let g = |s| "Hello!";
    let h = |s : Vec<bool>| (s.len()) as f64;
    trait Match {
        type R;
        fn q<T>(
            self,
            f: impl Fn(bool) -> usize,
            g: impl Fn(i8) -> &'static str,
            h: impl Fn(Vec<T>) -> f64,
        ) -> Self::R;
    }
    impl Match for bool {
        type R = usize;

        fn q<T>(
            self,
            f: impl Fn(bool) -> usize,
            g: impl Fn(i8) -> &'static str,
            h: impl Fn(Vec<T>) -> f64,
        ) -> Self::R {
            f(self)
        }
    }

    impl Match for i8 {
        type R = &'static str;

        fn q<T>(
            self,
            f: impl Fn(bool) -> usize,
            g: impl Fn(i8) -> &'static str,
            h: impl Fn(Vec<T>) -> f64,
        ) -> Self::R {
            g(self)
        }
    }

    impl<T> Match for Vec<T> {
        type R = f64;

        fn q<T = T>(
            self,
            f: impl Fn(bool) -> usize,
            g: impl Fn(i8) -> &'static str,
            h: impl Fn(Vec<T>) -> f64,
        ) -> Self::R {
            h(self)
        }
    } */
