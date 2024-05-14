#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = "1".to_string();

    ans.push_str(&"01".repeat(n));

    println!("{}", ans);
}
