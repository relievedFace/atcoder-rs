#![allow(non_snake_case)]
#![allow(unused_variables)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let ans = a.windows(2).map(|a| a[0] * a[1]).join(" ");

    println!("{}", ans);
}
