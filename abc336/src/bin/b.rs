#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        mut n: i32,
    }

    let mut ans = 0;

    while n & 1 == 0 {
        n >>= 1;
        ans += 1;
    }

    println!("{}", ans);
}
