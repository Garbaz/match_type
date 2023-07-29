use match_type::match_type;
use std::fmt::{Debug, Display};

// fn f<T>(x: T) -> String {
//     match_type! {
//         x {
//             <U: std::fmt::Display> U => String: format!("{}", self),
//             _ => String: "No Display :(".to_string(),
//         }
//     }
// }

// macro_rules! m {
//     ($e:expr) => {
//         match_type!{
//             $e {
//                 <T> Vec<T> => usize: self.len(),
//                 usize => f64: self as f64,
//                 f64 => bool: self.is_finite(),

//             }
//         }
//     };
// }

// fn g<T>(x: T) -> String {
//     match_type! {
//         x {
//             <D: Display> D => String: format!("{}", self),
//             _ => String: match_type! {
//                 self {
//                     <D: Debug> D => String: format!("{:?}", self),
//                     _ => String: "Sad Monkey :(",
//                 }
//             }
//         }
//     }
// }
// ^ Does not work without UUIDs

// macro_rules! m {
//     ($e:expr) => {
//         match_type! {
//             $e {
//                 <D: Display> D => String: format!("{}", self),
//                 <D: Debug> D => String: format!("{:?}", self),
//                 _ => bool: false,
//             }
//         }
//     };
// }

struct A;
struct B;
struct C;
struct D;

struct E;

macro_rules! m1 {
    ($e:expr) => {
        match_type!(
            $e {
                A => String: "A".to_string(),
                B => String: "B".to_string(),
                C => String: "C".to_string(),
                D => String: "D".to_string(),
                <T : Display> T => String: format!("{}", self),
                <T : Debug> T => String: format!("{:?}", self),
                _ => bool : false,
            }
        )
    };
}

fn main() {
    println!("{:?}", m1!(A));
    println!("{:?}", m1!(B));
    println!("{:?}", m1!(C));
    println!("{:?}", m1!(D));
    println!("{:?}", m1!(1729));
    println!("{:?}", m1!(vec![1,2,3,4]));
    println!("{:?}", m1!(E));

    // let q = match_type!{
    //     0.1f64 {
    //         <T> Vec<T> => usize: self.len(),
    //         usize => f64: self as f64,
    //         f64 => bool: self.is_finite(),
    //     }
    // };
    // let x : usize = m!(vec![1, 2, 3]);
    // let y : f64 = m!(x);
    // let z : bool = m!(y);

    // println!("{}", z);

    // println!("{}", 1729u32);
    // println!("{}", f(vec![1, 2, 3]));

    // let i = 1729;
    // let v = vec![1, 2, 3];
    // let s = S();

    // let x = m!(&i);
    // let y = m!(&v);
    // let z = m!(&s);

    // let _w = match_type! {
    //     3u32 {
    //         u32 => Vec<i32> : vec![1,2,3,4,5],
    //     }
    // };
}
