#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        mut sx: i64,
        mut sy: i64,
        mut tx: i64,
        mut ty: i64,
    }

    if (sx + sy) % 2 == 1 {
        sx -= 1;
    }

    if (tx + ty) % 2 == 1 {
        tx -= 1;
    }

    let dx = (sx - tx).abs();
    let dy = (sy - ty).abs();
    let ans = (dy + max(dx, dy)) / 2;

    println!("{}", ans);
}
