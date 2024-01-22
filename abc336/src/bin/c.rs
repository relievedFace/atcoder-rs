#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

const EVEN: [char; 5] = ['0', '2', '4', '6', '8'];

fn main() {
    input! {
        mut n: usize,
    }

    let mut ans = String::new();

    n -= 1;

    while n != 0 {
        ans.push(EVEN[n % 5]);
        n /= 5;
    }

    let mut ans: String = ans.chars().rev().collect();

    if ans.is_empty() {
        ans.push('0');
    }
    println!("{}", ans);
}
