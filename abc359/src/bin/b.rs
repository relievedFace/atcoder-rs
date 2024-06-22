#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n * 2],
    }

    let ans = a.windows(3).filter(|w| w[0] == w[2]).count();

    println!("{}", ans);
}
