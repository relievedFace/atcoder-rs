#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let ans = if s[0].is_uppercase() && s[1..].iter().all(|c| c.is_lowercase()) {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
