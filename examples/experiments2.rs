use std::fmt::{Debug, Display};

// Done

struct MatchTypeDone<T>(T);

impl<T> MatchTypeDone<T> {
    // fn match_type_w0(self) -> Self {
    //     self
    // }
    fn match_type_w1(self) -> Self {
        self
    }
    fn match_type_w2(self) -> Self {
        self
    }
    fn match_type_finish(self) -> T {
        self.0
    }
}

// 0

trait MatchTypeMatch0 {
    type MatchTypeReturnType;
    fn match_type_m0(self) -> Self::MatchTypeReturnType;
}

struct MatchTypeWrapper0<T>(T);

impl<T: MatchTypeMatch0> MatchTypeWrapper0<T> {
    fn match_type_w0(self) -> MatchTypeDone<<T as MatchTypeMatch0>::MatchTypeReturnType> {
        MatchTypeDone(MatchTypeMatch0::match_type_m0(self.0))
    }
}

trait MatchTypeCatch0 {
    type MatchTypeReturnType;
    fn match_type_w0(self) -> Self::MatchTypeReturnType;
}

impl<T> MatchTypeCatch0 for T {
    type MatchTypeReturnType = MatchTypeWrapper1<Self>;
    fn match_type_w0(self) -> Self::MatchTypeReturnType {
        MatchTypeWrapper1(self)
    }
}

impl<T: Display> MatchTypeMatch0 for T {
    type MatchTypeReturnType = String;
    fn match_type_m0(self) -> Self::MatchTypeReturnType {
        format!("{}", self)
    }
}

// 1

trait MatchTypeMatch1 {
    type MatchTypeReturnType;
    fn match_type_m1(self) -> Self::MatchTypeReturnType;
}

struct MatchTypeWrapper1<T>(T);

impl<T: MatchTypeMatch1> MatchTypeWrapper1<T> {
    fn match_type_w1(self) -> MatchTypeDone<<T as MatchTypeMatch1>::MatchTypeReturnType> {
        MatchTypeDone(MatchTypeMatch1::match_type_m1(self.0))
    }
}

trait MatchTypeCatch1 {
    type MatchTypeReturnType;
    fn match_type_w1(self) -> Self::MatchTypeReturnType;
}

impl<T> MatchTypeCatch1 for T {
    type MatchTypeReturnType = MatchTypeWrapper2<Self>;
    fn match_type_w1(self) -> Self::MatchTypeReturnType {
        MatchTypeWrapper2(self)
    }
}

impl<T: Debug> MatchTypeMatch1 for MatchTypeWrapper1<MatchTypeWrapper0<T>> {
    type MatchTypeReturnType = String;

    fn match_type_m1(self) -> Self::MatchTypeReturnType {
        format!("{:?}", self.0 .0)
    }
}

// 2

trait MatchTypeMatch2 {
    type MatchTypeReturnType;
    fn match_type_m2(self) -> Self::MatchTypeReturnType;
}

struct MatchTypeWrapper2<T>(T);

impl<T: MatchTypeMatch2> MatchTypeWrapper2<T> {
    fn match_type_w2(self) -> MatchTypeDone<<T as MatchTypeMatch2>::MatchTypeReturnType> {
        MatchTypeDone(MatchTypeMatch2::match_type_m2(self.0))
    }
}

trait MatchTypeCatch2 {
    type MatchTypeReturnType;
    fn match_type_w2(self) -> Self::MatchTypeReturnType;
}

impl<T> MatchTypeCatch2 for T {
    type MatchTypeReturnType = MatchTypeWrapper3<Self>;
    fn match_type_w2(self) -> Self::MatchTypeReturnType {
        MatchTypeWrapper3(self)
    }
}

impl<T> MatchTypeMatch2 for MatchTypeWrapper2<MatchTypeWrapper1<T>> {
    type MatchTypeReturnType = &'static str;

    fn match_type_m2(self) -> Self::MatchTypeReturnType {
        "Sad Monkey :("
    }
}

struct MatchTypeWrapper3<T>(T);

impl<T> MatchTypeWrapper3<T> {
    fn match_type_finish(self) -> ! {
        unimplemented!()
    }
}

//===============================================//

macro_rules! m {
    ($e:expr) => {
        (MatchTypeWrapper0($e)
            .match_type_w0()
            .match_type_w1()
            .match_type_w2()
            .match_type_finish())
    };
}

struct S();

fn main() {
    let q = MatchTypeWrapper0(vec![1, 2, 3])
        .match_type_w0()
        .match_type_w1()
        .match_type_w2();
    // .match_type_finish();

    let i = 1729;
    let v = vec![0, 1, 2];
    let s = S();

    let x = m!(&i);
    let y = m!(&v);
    let z = m!(&s);

    // println!("{}\n{}\n{}", x, y, z);
    println!("{:?}", &v);
}
