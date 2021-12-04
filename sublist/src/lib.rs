use crate::Comparison::{Equal, Sublist, Superlist, Unequal};
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Display>(l1: &[T], l2: &[T]) -> Comparison {
    match (l1.len(), l2.len()) {
        (0, 0) => Equal,
        (_, 0) => Superlist,
        (0, _) => Sublist,
        (x, y) if x > y => if l1.windows(l2.len()).any(|chunk| chunk == l2) { Superlist } else { Unequal }
        (x, y) if y > x => if l2.windows(l1.len()).any(|chunk| chunk == l1) { Sublist } else { Unequal },
        (x, y) if x == y => if l2.windows(l1.len()).any(|chunk| chunk == l1) { Equal } else { Unequal },
        _ => panic!("Not possible!")
    }
}

// Let length of smaller list be m
// Let length of bigget list be n
// Time complexity: O((n-m)*m)
// Essentially same time complexity as the windows method by non-idiomatic
// fn sublist_iter<T: PartialEq + Display>(l1: &[T], l2: &[T]) -> bool {
//     for i in 0..l2.len()-l1.len()+1 {
//         let mut hit = true;
//         //
//         // println!("i: {}", i);
//         //
//         // for x in &l2[i..l1.len()+i] {
//         //     print!("{}, ", x);
//         // }
//         // println!();
//
//         for j in 0..l1.len() {
//             if l1[j] == l2[i+j] {
//                 continue;
//             } else {
//                 hit = false;
//                 break;
//             }
//         }
//         if hit {
//             return true;
//         }
//     }
//
//     false
// }

// fn is_sublist_rec<T: PartialEq>(l1: &[T], l2: &[T], i: usize, j: usize) -> bool {
//     if i < l1.len() {
//         return match &l2[j..] {
//             [x, ..] => {
//                 if &l1[i] != x {
//                     is_sublist_rec(l1, l2, 0, j+1)
//                 } else {
//                     is_sublist_rec(l1, l2, i+1, j+1) || is_sublist_rec(l1, l2, i, j+1)
//                 }
//             },
//             _ => false // Reached end of l2
//         };
//     }
//
//     true
// }
