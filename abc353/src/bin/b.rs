#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        a: [i32; n],
    }

    let mut attraction = 0;
    let mut ans = 0;

    for a in a {
        if attraction + a > k {
            attraction = a;
            ans += 1;
        } else {
            attraction += a;
        }
    }

    if attraction != 0 {
        ans += 1;
    }

    println!("{}", ans);
}
