#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let s3: Vec<&str> = s.split('|').collect();
    let ans = format!("{}{}", s3[0], s3[2]);

    println!("{}", ans);
}
