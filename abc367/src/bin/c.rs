#![allow(non_snake_case)]
#![allow(unused_variables)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        r: [i32; n]
    }

    let mut ans = vec![vec![]];
    make_pattern(n, k, &r, 0, vec![], &mut ans);
    ans.sort();

    let ans = ans.iter().map(|l| l.iter().join(" ")).join("¥n");

    println!("{}", ans);
}

fn make_pattern(
    n: usize,
    k: i32,
    r: &Vec<i32>,
    count: usize,
    tmp: Vec<i32>,
    ans: &mut Vec<Vec<i32>>,
) {
    if n == count {
        if tmp.iter().sum::<i32>() % k == 0 {
            ans.push(tmp.clone());
        }
        return;
    }

    for i in 1..=r[count] {
        let mut tmp = tmp.clone();
        tmp.push(i);
        make_pattern(n, k, r, count + 1, tmp, ans);
    }
}
