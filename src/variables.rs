use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;

fn main(){
    println!("Hello world!");

    let num: i32 = 10;

    let mut age: i32 = 40;

    let is_it_true: bool = true;

    let let_x: char = 'x';

    println!("I am {} years old!", age);

    let (f_name, l_name) = ("Derek", "Bananas");

    println!("It is  {0} that {1} is {0}", is_it_true, let_x);


    println!("{:.2}", 1.234);

    println!("B: {:b} H: {:x} 0: {:o}", 10, 10, 10);

    println!("{ten:>ws$}", ten=10, ws=5);

    println!("{ten:>0ws$}", ten=10, ws=5)
//    println!("Min i8 {}", i8::MIN);
//    println!("Max i8 {}", i8::MAX);
//    println!("Min i16 {}", i16::MIN);
//    println!("Max i16 {}", i16::MAX);
//    println!("Min i32 {}", i32::MIN);
//    println!("Max i32 {}", i32::MAX);
//    println!("Min i64 {}", i64::MIN);
//    println!("Max i64 {}", i64::MAX);
//    println!("Min isize {}", isize::MIN);
//    println!("Max isize {}", isize::MAX);
//    println!("Min usize {}", usize::MIN);
//    println!("Max usize {}", usize::MAX);
//    println!("Min f64 {}", f64::MIN);
//    println!("Max f64 {}", f64::MAX);
}