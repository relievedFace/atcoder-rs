#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        x: String
    }

    let ans = x.trim_end_matches('0');
    let ans = ans.trim_end_matches('.');

    println!("{}", ans);
}
