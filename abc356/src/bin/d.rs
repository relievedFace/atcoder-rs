#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        n: u64,
        m: u64,
    }

    let mut ans = 0;

    for k in 0..n {
        ans += (k & m).count_ones();
        ans %= 998244353;
    }

    println!("{}", ans);
}
