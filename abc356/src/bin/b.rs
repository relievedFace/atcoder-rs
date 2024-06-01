#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
        x: [[usize; m]; n],
    }

    let mut sum = vec![0; m];

    for i in 0..m {
        for j in 0..n {
            sum[i] += x[j][i];
        }
    }

    let ans = if a.iter().zip(sum.iter()).all(|(i, j)| i <= j) {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
