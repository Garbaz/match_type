use std::fmt::{Debug, Display};

// 1

trait Match1 {
    type ReturnType;
    fn m1(self) -> Self::ReturnType;
}

struct Wrapper1<T>(T);

impl<T: Match1> Wrapper1<T> {
    fn w1(self) -> Done<<T as Match1>::ReturnType> {
        Done(Match1::m1(self.0))
    }
}

trait Catch1 {
    type ReturnType;
    fn w1(self) -> Self::ReturnType;
}

impl<T> Catch1 for T {
    type ReturnType = Wrapper2<Self>;
    fn w1(self) -> Self::ReturnType {
        Wrapper2(self)
    }
}

// 2

trait Match2 {
    type ReturnType;
    fn m2(self) -> Self::ReturnType;
}

struct Wrapper2<T>(T);

impl<T: Match2> Wrapper2<T> {
    fn w2(self) -> Done<<T as Match2>::ReturnType> {
        Done(Match2::m2(self.0))
    }
}

trait Catch2 {
    type ReturnType;
    fn w2(self) -> Self::ReturnType;
}

impl<T> Catch2 for T {
    type ReturnType = Wrapper3<Self>;
    fn w2(self) -> Self::ReturnType {
        Wrapper3(self)
    }
}

struct Done<T>(T);

impl<T> Done<T> {
    fn w1(self) -> Self {
        self
    }
    fn w2(self) -> Self {
        self
    }
    fn w3(self) -> Self {
        self
    }
    fn fin(self) -> T {
        self.0
    }
}

// 3

trait Match3 {
    type ReturnType;
    fn m3(self) -> Self::ReturnType;
}

struct Wrapper3<T>(T);

impl<T: Match3> Wrapper3<T> {
    fn w3(self) -> Done<<T as Match3>::ReturnType> {
        Done(Match3::m3(self.0))
    }
}

//===============================================//

impl<T: Display> Match1 for T {
    type ReturnType = String;
    fn m1(self) -> Self::ReturnType {
        format!("{}", self)
    }
}

impl<T: Debug> Match2 for Wrapper1<T> {
    type ReturnType = String;

    fn m2(self) -> Self::ReturnType {
        format!("{:?}", self.0)
    }
}

impl<T> Match3 for T {
    type ReturnType = &'static str;

    fn m3(self) -> Self::ReturnType {
        "Sad Monkey :("
    }
}

macro_rules! m {
    ($e:expr) => {
        Wrapper1($e).w1().w2().w3().fin()
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
