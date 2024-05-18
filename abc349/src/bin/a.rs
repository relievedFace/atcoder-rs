#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n - 1],
    }

    let sum: i32 = a.iter().sum();
    let ans = -sum;

    println!("{}", ans);
}
