#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let ans = s.iter().filter(|&s| s == "Takahashi").count();

    println!("{}", ans);
}
