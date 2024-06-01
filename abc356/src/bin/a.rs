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

    let mut ans: Vec<_> = (1..=n).collect();

    ans[l - 1..r].reverse();

    let ans = ans.iter().join(" ");

    println!("{}", ans);
}
