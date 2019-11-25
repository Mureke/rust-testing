use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::{stdin, Read};

fn main() {
    say_hello("Derek");
    println!("5 + 3 = {}", get_sum(5, 3));

    let sum = get_sum;
    println!("6 + 4 = {}", sum(6, 4));

    let sum_nums = |x: i32, y: i32 | x + y;
    println!("7 + 22 = {}", sum_nums(7,22));

    let num_ten = 10;
    let add_10 = |x: i32| x + num_ten;
    println!("5 + 10 = {}", add_10(5))

}

fn say_hello(name: &str) {
    println!("Hello {}", name);
}


fn get_sum(n: i32, n2: i32) -> i32 {
    n + n2
}