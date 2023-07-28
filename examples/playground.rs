use match_type::match_type;


fn f<T>(x : T) -> String {
    match_type!{
        x {
            <U: std::fmt::Display> U => String: format!("{}", self),
            _ => String: "No Display :(".to_string(),
        }
    }
}

macro_rules! m {
    ($e:expr) => {
        match_type!{
            $e {
                <T> Vec<T> => usize: self.len(),
                usize => f64: self as f64,
                f64 => bool: self.is_finite(),

            }
        }
    };
}

fn main() {
    let x = m!(vec![1,2,3]);
    let y = m!(x);
    let z = m!(y);

    println!("{}", z);

    println!("{}", 1729u32);
    println!("{}", f(vec![1,2,3]));
}
