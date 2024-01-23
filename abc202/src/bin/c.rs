#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::{input, marker::Usize1};
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        b: [Usize1; n],
        c: [Usize1; n],
    }

    let mut map = BTreeMap::new();

    for i in 0..n {
        *map.entry(b[c[i]]).or_insert(0) += 1;
    }

    let mut ans = 0usize;

    for ai in a.iter() {
        ans += map.get(ai).unwrap_or(&0);
    }

    println!("{}", ans);
}
