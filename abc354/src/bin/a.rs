#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        h: usize,
    }

    let mut plant = 1;
    let mut ans = 1;

    while plant <= h {
        ans += 1;
        plant *= 2;
        plant += 1;
    }

    println!("{}", ans);
}
