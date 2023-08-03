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

struct W0<T>(T);

impl W0<A> {
    fn w0(self) -> Done<String> {
        println!("arm 0 succeeded");
        Done("A!".to_string())
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

struct W1<T>(T);

impl W1<B> {
    fn w1(self) -> Done<&'static str> {
        println!("arm 1 succeeded");
        Done("B it is.")
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

struct W2<T>(T);

impl W2<C> {
    fn w2(self) -> Done<u64> {
        println!("arm 2 succeeded");
        Done(1729)
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
