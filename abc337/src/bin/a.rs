#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }

    let xの合計: i32 = xy.iter().map(|(x, _)| x).sum();
    let yの合計: i32 = xy.iter().map(|(_, y)| y).sum();

    let 答え = match xの合計.cmp(&yの合計) {
        std::cmp::Ordering::Less => "Aoki",
        std::cmp::Ordering::Equal => "Draw",
        std::cmp::Ordering::Greater => "Takahashi",
    };

    println!("{}", 答え)
}
