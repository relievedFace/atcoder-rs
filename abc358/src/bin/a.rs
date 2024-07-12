#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let ans = if s == "AtCoder" && t == "Land" {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
