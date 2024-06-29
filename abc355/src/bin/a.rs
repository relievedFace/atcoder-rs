#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    let ans = if a == b {
        -1
    } else {
        *vec![1, 2, 3].iter().find(|x| **x != a && **x != b).unwrap()
    };

    println!("{}", ans);
}
