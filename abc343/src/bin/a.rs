#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    let ans = (a + b + 1) % 10;

    println!("{}", ans);
}
