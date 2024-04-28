#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::{input, marker::Chars};
use std::{cmp::Reverse, collections::HashMap, hash::Hash};

fn main() {
    input! {
        s: Chars,
    }

    let map = convert_vec_to_hashmap(&s);

    let ans = ('a'..='z')
        .map(|c| (c, map.get(&c).unwrap_or(&0)))
        .max_by_key(|k| (k.1, Reverse(k.0)))
        .unwrap()
        .0;

    println!("{}", ans);
}

pub fn convert_vec_to_hashmap<T>(vec: &[T]) -> HashMap<T, usize>
where
    T: Clone + Eq + Hash,
{
    vec.iter().cloned().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    })
}
