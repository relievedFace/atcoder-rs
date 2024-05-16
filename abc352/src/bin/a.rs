#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;
use std::mem::swap;

fn main() {
    input! {
        n: i32,
        mut x: i32,
        mut y: i32,
        z: i32,
    }

    if x > y {
        swap(&mut x, &mut y);
    }

    let ans = if x < z && z < y { "Yes" } else { "No" };

    println!("{}", ans);
}
