#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i32; n],
    }

    let ans = if let Some(h) = h.iter().enumerate().find(|(i, hi)| **hi > h[0]) {
        h.0 as i32 + 1
    } else {
        -1
    };

    println!("{}", ans);
}
