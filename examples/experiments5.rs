use std::fmt::{Debug, Display};

struct D<T>(T);

impl<T> D<T> {
    fn arm0(self) -> Self {
        self
    }
    fn arm1(self) -> Self {
        self
    }
    fn arm2(self) -> Self {
        self
    }
    fn match_arm_found(self) -> T {
        self.0
    }
}

struct W<T>(T);

trait C<T> {
    fn arm0(self) -> Self;
    fn arm1(self) -> Self;
    fn arm2(self) -> Self;
}

impl<T> C<T> for W<T> {
    fn arm0(self) -> Self {
        self
    }

    fn arm1(self) -> Self {
        self
    }

    fn arm2(self) -> Self {
        self
    }
}

impl W<A> {
    fn arm0(self) -> D<&'static str> {
        D("Wohoo!")
    }
}

impl<T: Display> W<T> {
    fn arm1(self) -> D<String> {
        D(format!("{}", self.0))
    }
}

impl<T: Debug> W<T> {
    fn arm2(self) -> D<String> {
        D(format!("{:?}", self.0))
    }
}

struct A;

struct B;

fn main() {
    let x = A;
    let y = 1729;
    let z = vec![1, 2, 3];

    println!("{}", W(x).arm0().arm1().arm2().match_arm_found());
    println!("{}", W(y).arm0().arm1().arm2().match_arm_found());
    println!("{}", W(z).arm0().arm1().arm2().match_arm_found());
    // println!("{}", W(B).arm0().arm1().arm2().match_arm_found());
}
