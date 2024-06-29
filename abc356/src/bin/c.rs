#![allow(non_snake_case)]
#![allow(unused_variables)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let mut c = vec![];
    let mut a = vec![];
    let mut r = vec![];

    for _ in 0..m {
        input! {
            c_in: usize
        }
        c.push(c_in);

        input! {
            a_in: [usize; c_in],
        }

        a.push(a_in);

        input! {
            r_in: String,
        }

        r.push(r_in);
    }

    let mut ans = 0;

    for bit in 0..1 << n {
        let mut p = vec![false; n];
        for (i, p_i) in p.iter_mut().enumerate() {
            *p_i = (bit & (1 << i)) != 0;
        }

        let mut flag = true;

        for i in 0..m {
            let a = &a[i];
            let r = &r[i];

            let mut count = 0;

            for a in a {
                if p[a - 1] {
                    count += 1;
                }
            }

            if !((count >= k && r == "o") || (count < k && r == "x")) {
                flag = false;
                break;
            }
        }

        if flag {
            ans += 1;
        }
    }

    println!("{}", ans);
}
