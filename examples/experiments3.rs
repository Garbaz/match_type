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
    fn done(self) -> T {
        self.0
    }
}

struct A;
struct B;
struct C;
struct D;

trait M0 {
    type R;
    fn e0(self) -> Self::R;
}

impl M0 for A {
    type R = String;

    fn e0(self) -> Self::R {
        "A".to_string()
    }
}

struct W0<T>(T);

impl<T: M0> W0<T> {
    fn w0(self) -> Done<<T as M0>::R> {
        println!("arm 0 succeeded");
        Done(M0::e0(self.0))
    }
}
trait C0 {
    type R;
    fn w0(self) -> Self::R;
}
impl<T> C0 for W0<T> {
    type R = W1<T>;
    fn w0(self) -> Self::R {
        println!("arm 0 failed");
        W1(self.0)
    }
}

//=========================

trait M1 {
    type R;
    fn e1(self) -> Self::R;
}

impl M1 for B {
    type R = String;

    fn e1(self) -> Self::R {
        "B".to_string()
    }
}

struct W1<T>(T);

impl<T: M1> W1<T> {
    fn w1(self) -> Done<<T as M1>::R> {
        println!("arm 1 succeeded");
        Done(M1::e1(self.0))
    }
}
trait C1 {
    type R;
    fn w1(self) -> Self::R;
}
impl<T> C1 for W1<T> {
    type R = W2<T>;
    fn w1(self) -> Self::R {
        println!("arm 1 failed");
        W2(self.0)
    }
}

//=========================

trait M2 {
    type R;
    fn e2(self) -> Self::R;
}

impl M2 for C {
    type R = String;

    fn e2(self) -> Self::R {
        "C".to_string()
    }
}

struct W2<T>(T);

impl<T: M2> W2<T> {
    fn w2(self) -> Done<<T as M2>::R> {
        println!("arm 2 succeeded");
        Done(M2::e2(self.0))
    }
}
trait C2 {
    type R;
    fn w2(self) -> Self::R;
}
impl<T> C2 for W2<T> {
    type R = W3<T>;
    fn w2(self) -> Self::R {
        println!("arm 2 failed");
        W3(self.0)
    }
}

struct W3<T>(T);

impl<T> W3<T> {
    fn done(self) -> () {
        ()
    }
}

fn main() {
    println!("{:?}", W0(A).w0().w1().w2().done());
    println!("{:?}", W0(B).w0().w1().w2().done());
    println!("{:?}", W0(C).w0().w1().w2().done());
    println!("{:?}", W0(D).w0().w1().w2().done());
}
