#![allow(non_snake_case)]
#![allow(unused_variables)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let r_pos = s.chars().find_position(|&c| c == 'R').unwrap();
    let m_pos = s.chars().find_position(|&c| c == 'M').unwrap();

    let ans = if m_pos > r_pos { "Yes" } else { "No" };

    println!("{}", ans);
}
