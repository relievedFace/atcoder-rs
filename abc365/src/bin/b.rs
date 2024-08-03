#![allow(non_snake_case)]
#![allow(unused_variables)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let ans = a
        .iter()
        .enumerate()
        .sorted_by_key(|k| k.1)
        .rev()
        .nth(1)
        .unwrap()
        .0
        + 1;

    println!("{}", ans);
}
