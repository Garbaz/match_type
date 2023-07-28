trait Match {
    type ReturnType;
    fn arm(&self) -> Self::ReturnType;
}

struct Wrapper<T>(T);

impl<T: Match> Wrapper<T> {
    fn arm(&self) -> <T as Match>::ReturnType {
        self.0.arm()
    }
}

type DefaultReturnType = usize;

trait CatchMatch {
    fn arm(&self) -> DefaultReturnType;
}

impl<T> CatchMatch for T {
    fn arm(&self) -> DefaultReturnType {
        1729
    }
}

impl Match for i8 {
    type ReturnType = bool;
    fn arm(&self) -> Self::ReturnType {
        false
    }
}

impl Match for bool {
    type ReturnType = i8;

    fn arm(&self) -> Self::ReturnType {
        -3
    }
}

fn test() {
    let x = Wrapper(false).arm();
    let y = Wrapper(-3i8).arm();
    let z = Wrapper("hello").arm();
}
