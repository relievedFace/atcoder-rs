#![allow(non_snake_case)]
#![allow(unused_variables)]

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let map = convert_vec_to_hashmap(&s);
    let c1 = map.iter().find(|(c, i)| **i == 1).unwrap().0;
    let ans = s.iter().find_position(|c| *c == c1).unwrap().0 + 1;

    println!("{}", ans);
}

use std::{collections::HashMap, hash::Hash};

pub fn convert_vec_to_hashmap<T>(vec: &[T]) -> HashMap<T, usize>
where
    T: Clone + Eq + Hash,
{
    vec.iter().cloned().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    })
}
