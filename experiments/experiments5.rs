use std::fmt::{Debug, Display};

struct Done<T>(T);

impl<T> Done<T> {
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

struct Wrapper<T>(T);

trait Catch<T> {
    fn arm0(self) -> Self;
    fn arm1(self) -> Self;
    fn arm2(self) -> Self;
}

impl<T> Catch<T> for Wrapper<T> {
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

impl Wrapper<A> {
    fn arm0(self) -> Done<&'static str> {
        Done("Wohoo!")
    }
}

impl<T: Display> Wrapper<T> {
    fn arm1(self) -> Done<String> {
        Done(format!("{}", self.0))
    }
}

impl<T: Debug> Wrapper<T> {
    fn arm2(self) -> Done<String> {
        Done(format!("{:?}", self.0))
    }
}

struct A;

fn main() {
    let x = A;
    let y = 1729;
    let z = vec![1, 2, 3];

    println!("{}", Wrapper(x).arm0().arm1().arm2().match_arm_found());
    println!("{}", Wrapper(y).arm0().arm1().arm2().match_arm_found());
    println!("{}", Wrapper(z).arm0().arm1().arm2().match_arm_found());
    // println!("{}", W(B).arm0().arm1().arm2().match_arm_found());
}
