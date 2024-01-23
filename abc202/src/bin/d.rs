#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
        mut k: usize,
    }

    let mut dp = vec![vec![0; b + 1]; a + 1];

    dp[0][0] = 1;

    for i in 0..=a {
        for j in 0..=b {
            if i > 0 {
                dp[i][j] += dp[i - 1][j];
            }

            if j > 0 {
                dp[i][j] += dp[i][j - 1];
            }
        }
    }

    while a > 0 && b > 0 {
        if k <= dp[a - 1][b] {
            print!("a");
            a -= 1;
        } else {
            k -= dp[a - 1][b];
            print!("b");
            b -= 1;
        }
    }
    println!("{}{}", "a".repeat(a), "b".repeat(b));
}
