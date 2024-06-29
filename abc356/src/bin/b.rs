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

    for (x_i, sum_i) in x.iter().zip(sum.iter_mut()) {
        for x_ij in x_i.iter() {
            *sum_i += x_ij;
        }
    }

    let ans = if a.iter().zip(sum.iter()).all(|(i, j)| i <= j) {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
