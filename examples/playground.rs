use match_type::match_type;
use std::fmt::{Debug, Display};
use std::ops::Neg;
use std::ops::Not;

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

macro_rules! stringify {
    ($e:expr) => {
        match_type!(
            $e {
                A => String: "It's an A :O".to_string(),
                B => String: "B it is ^^".to_string(),
                <T : Display> T => String: format!("{}", self),
                <T : Debug> T => String: format!("{:?}", self),
                _ => String : "<<Sad Monkey :(>>".to_string(),
            }
        )
    }
}

// macro_rules! sqrt {
//     ($e:expr) => {
//         match_type!(
//             $e {
//                 f64 => f64: self.sqrt(),
//                 f32 => f32: self.sqrt(),
//                 u8  => u8: ((self as f64).sqrt() as u8),
//                 u16  => u16: ((self as f64).sqrt() as u16),
//                 u32  => u32: ((self as f64).sqrt() as u32),
//                 u64  => u64: ((self as f64).sqrt() as u64),
//             }
//         )
//     }
// }

macro_rules! inv {
    ($e:expr) => {
        match_type!(
            $e {
                <T: Neg> T => <T as Neg>::Output: -self,
                <T: Not> T => <T as Not>::Output: !self,
            }
        )
    };
}

fn main() {
    println!("{}", inv!(true));
    println!("{}", inv!(17));
    // println!("{}", sqrt!(1729u32));
    // println!("{}", sqrt!(1729i16));
    // println!("{}", sqrt!(1729.0f32));
    // println!("{}", sqrt!(1729.0f64));
    
    // println!("{:?}", stringify!(A));
    // println!("{:?}", stringify!(B));
    // println!("{:?}", stringify!(1729));
    // println!("{:?}", stringify!(vec![1,2,3,4]));
    // println!("{:?}", stringify!(C));
    // let x = 10;
    // println!("{}", ((x as f64).sqrt() as u8))

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
