use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;

fn main() {
    let rand_str = "I am a random string";

    let (first, second) = rand_str.split_at(6);

    println!("first: {} second {}", first, second);

    let count = rand_str.chars().count();

    let mut chars = rand_str.chars();

    let mut invid_char = chars.next();


    loop {
        match invid_char {
            Some(x) => println!("{}", x),
            None => break,
        }
        invid_char = chars.next()
    }

    let mut iter = rand_str.split_whitespace();

    let mut indiv_word = iter.next();

    loop {
        match indiv_word {
            Some(x) => println!("{}", x),
            None => break,
        }
        indiv_word = iter.next();
    }

    let rand_str2 = "Iam random string\nThere are other strings like it.\nThis string is the best";

    let mut lines = rand_str2.lines();
    let mut indiv_l = lines.next();

    loop {
        match indiv_l {
            Some(x) => println!("{}", x),
            None => break,
        }
        indiv_l = lines.next();
    }

    println!("Find best: {}", rand_str2.contains("best"))
}