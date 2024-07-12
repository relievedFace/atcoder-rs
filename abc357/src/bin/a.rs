#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: i32,
        h: [i32; n],
    }

    let mut ans = n;

    for (i, hi) in h.iter().enumerate() {
        m -= hi;

        match m.cmp(&0) {
            std::cmp::Ordering::Less => {
                ans = i;
                break;
            }
            std::cmp::Ordering::Equal => {
                ans = i + 1;
                break;
            }
            std::cmp::Ordering::Greater => (),
        }
    }

    println!("{}", ans);
}
