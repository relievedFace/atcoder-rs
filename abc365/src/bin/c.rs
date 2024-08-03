#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let a_sum: usize = a.iter().sum();

    if a_sum <= m {
        println!("infinite");
        return;
    }

    let mut imin = 0;
    let mut imax = m;

    while imax - imin > 1 {
        let imid = imin + (imax - imin) / 2;
        let sum: usize = a.iter().map(|ai| min(ai, &imid)).sum();

        if sum > m {
            imax = imid;
        } else {
            imin = imid;
        }
    }

    println!("{}", imin);
}
