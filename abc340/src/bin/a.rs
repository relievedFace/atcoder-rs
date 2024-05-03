#![allow(non_snake_case)]
#![allow(unused_variables)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut a: i32,
        b: i32,
        d: i32,
    }

    let ans = (0..=(b - a) / d).map(|x| a + d * x).join(" ");

    println!("{}", ans);
}
