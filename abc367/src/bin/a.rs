#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        a: i32,
        mut b: i32,
        c: i32,
    }

    let mut cur = b;
    while cur != c {
        if cur == a {
            println!("No");
            return;
        }

        cur += 1;
        cur %= 24;

        if cur == a {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
