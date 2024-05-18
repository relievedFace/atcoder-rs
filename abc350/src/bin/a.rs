#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let s_is_start_with_abc = s.starts_with("ABC");
    let number: String = s.chars().skip(3).collect();

    let ans = if s_is_start_with_abc
        && "001" <= number.as_str()
        && number.as_str() <= "349"
        && &number != "316"
    {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
