#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars
    }

    let ans = if s.first() == Some(&'<')
        && s.last() == Some(&'>')
        && s[1..s.len() - 1].iter().all(|c| *c == '=')
    {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
