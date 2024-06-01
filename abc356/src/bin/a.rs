#![allow(non_snake_case)]
#![allow(unused_variables)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize
    }

    let vec_n: Vec<_> = (1..=n).collect();
    let left: Vec<_> = vec_n[..l - 1].to_vec();
    let mid: Vec<_> = vec_n[l - 1..r].iter().rev().collect();
    let right: Vec<_> = vec_n[r..].to_vec();
    let mut ans = left.clone();
    ans.extend(mid);
    ans.extend(right);
    let ans = ans.iter().join(" ");

    println!("{}", ans);
}
