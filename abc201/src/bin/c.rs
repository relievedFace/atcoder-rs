#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut ans = 0;

    for i in 0..=9999 {
        let mut flag = [false; 10];
        let mut x = i;

        for j in 0..4 {
            flag[x % 10] = true;
            x /= 10;
        }

        let mut flag2 = true;

        for j in 0..10 {
            if s[j] == 'o' && !flag[j] {
                flag2 = false
            };
            if s[j] == 'x' && flag[j] {
                flag2 = false
            };
        }

        if flag2 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
