#![allow(non_snake_case)]
#![allow(unused_variables)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    }

    let mut grid = vec![vec!['.'; w]; h];

    // 上: 0
    // 右: 1
    // 下: 2
    // 左: 3
    let mut status = 0;
    let mut x = 0;
    let mut y = 0;

    for _ in 0..n {
        if grid[y][x] == '.' {
            grid[y][x] = '#';
            status += 1;
            status %= 4;
            match status {
                0 if y == 0 => y = h - 1,
                0 => y -= 1,
                1 if x == w - 1 => x = 0,
                1 => x += 1,
                2 if y == h - 1 => y = 0,
                2 => y += 1,
                3 if x == 0 => x = w - 1,
                3 => x -= 1,
                _ => unreachable!(),
            }
        } else {
            grid[y][x] = '.';
            if status == 0 {
                status = 3;
            } else {
                status -= 1;
            }
            match status {
                0 if y == 0 => y = h - 1,
                0 => y -= 1,
                1 if x == w - 1 => x = 0,
                1 => x += 1,
                2 if y == h - 1 => y = 0,
                2 => y += 1,
                3 if x == 0 => x = w - 1,
                3 => x -= 1,
                _ => unreachable!(),
            }
        }
    }

    let ans = grid.iter().map(|l| l.iter().collect::<String>()).join("\n");

    println!("{}", ans);
}
