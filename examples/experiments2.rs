use std::fmt::{Debug, Display};

// Done

struct __MatchTypeDone<T>(T);

impl<T> __MatchTypeDone<T> {
    fn __match_type_w1(self) -> Self {
        self
    }
    fn __match_type_w2(self) -> Self {
        self
    }
    fn __match_type_w3(self) -> Self {
        self
    }
    fn __match_type_finish(self) -> T {
        self.0
    }
}

// 1

trait __MatchTypeMatch1 {
    type __MatchTypeReturnType;
    fn __match_type_m1(self) -> Self::__MatchTypeReturnType;
}

struct __MatchTypeWrapper1<T>(T);

impl<T: __MatchTypeMatch1> __MatchTypeWrapper1<T> {
    fn __match_type_w1(self) -> __MatchTypeDone<<T as __MatchTypeMatch1>::__MatchTypeReturnType> {
        __MatchTypeDone(__MatchTypeMatch1::__match_type_m1(self.0))
    }
}

trait Catch1 {
    type __MatchTypeReturnType;
    fn __match_type_w1(self) -> Self::__MatchTypeReturnType;
}

impl<T> Catch1 for T {
    type __MatchTypeReturnType = __MatchTypeWrapper2<Self>;
    fn __match_type_w1(self) -> Self::__MatchTypeReturnType {
        __MatchTypeWrapper2(self)
    }
}

// 2

trait __MatchTypeMatch2 {
    type __MatchTypeReturnType;
    fn __match_type_m2(self) -> Self::__MatchTypeReturnType;
}

struct __MatchTypeWrapper2<T>(T);

impl<T: __MatchTypeMatch2> __MatchTypeWrapper2<T> {
    fn __match_type_w2(self) -> __MatchTypeDone<<T as __MatchTypeMatch2>::__MatchTypeReturnType> {
        __MatchTypeDone(__MatchTypeMatch2::__match_type_m2(self.0))
    }
}

trait Catch2 {
    type __MatchTypeReturnType;
    fn __match_type_w2(self) -> Self::__MatchTypeReturnType;
}

impl<T> Catch2 for T {
    type __MatchTypeReturnType = __MatchTypeWrapper3<Self>;
    fn __match_type_w2(self) -> Self::__MatchTypeReturnType {
        __MatchTypeWrapper3(self)
    }
}

// 3

trait __MatchTypeMatch3 {
    type __MatchTypeReturnType;
    fn __match_type_m3(self) -> Self::__MatchTypeReturnType;
}

struct __MatchTypeWrapper3<T>(T);

impl<T: __MatchTypeMatch3> __MatchTypeWrapper3<T> {
    fn __match_type_w3(self) -> __MatchTypeDone<<T as __MatchTypeMatch3>::__MatchTypeReturnType> {
        __MatchTypeDone(__MatchTypeMatch3::__match_type_m3(self.0))
    }
}

//===============================================//

impl<T: Display> __MatchTypeMatch1 for T {
    type __MatchTypeReturnType = String;
    fn __match_type_m1(self) -> Self::__MatchTypeReturnType {
        format!("{}", self)
    }
}

impl<T: Debug> __MatchTypeMatch2 for __MatchTypeWrapper1<T> {
    type __MatchTypeReturnType = String;

    fn __match_type_m2(self) -> Self::__MatchTypeReturnType {
        format!("{:?}", self.0)
    }
}

impl<T> __MatchTypeMatch3 for T {
    type __MatchTypeReturnType = &'static str;

    fn __match_type_m3(self) -> Self::__MatchTypeReturnType {
        "Sad Monkey :("
    }
}

macro_rules! m {
    ($e:expr) => {
        (__MatchTypeWrapper1($e)
            .__match_type_w1()
            .__match_type_w2()
            .__match_type_w3()
            .__match_type_finish())
    };
}

//===============================================//

struct S();

fn main() {
    let i = 1729;
    let v = vec![1, 2, 3];
    let s = S();

    let x = m!(&i);
    let y = m!(&v);
    let z = m!(&s);

    println!("{}\n{}\n{}", x, y, z);
}
