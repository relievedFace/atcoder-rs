#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ans = format!("L{}ng", "o".repeat(n));

    println!("{}", ans);
}
