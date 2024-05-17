#![allow(non_snake_case)]
#![allow(unused_variables)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        a: [i32; n],
    }

    let ans = a.iter().filter(|a| **a % k == 0).map(|a| a / k).join(" ");

    println!("{}", ans);
}
