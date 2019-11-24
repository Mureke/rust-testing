use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;

fn main() {
    let mut x = 1;

    loop {
        if x % 2 == 0 {
            println!("{}", x);
            x += 1;
            continue;
        }
        if x > 10 {
            break;
        }
        x += 1;
        continue;
    }

    let mut y = 1;

    while y <= 10 {
        println!("{}", y);
        y += 1
    }

    for z in 0..11 {
        println!("{}", z);
    }
}