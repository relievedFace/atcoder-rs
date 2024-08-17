#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: i64,
        a: [i64; n],
    }

    let a2 = [a.clone(), a.clone()].concat();
    let mut sum = vec![0; 2 * n + 1];

    for i in 0..2 * n + 1 {
        sum[i + 1] = sum[i] + a2[i];
    }
}
