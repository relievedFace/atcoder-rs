#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut count = 0;
    let mut min = 0;

    for ai in a.iter() {
        count += ai;
        min = min.min(count);
    }

    let ans = count - min;

    println!("{}", ans);
}
