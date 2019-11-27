use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::{stdin, Read};

mod functions;
use functions::say_hello;

mod test;
use test::testisetti::say_hello2;


fn main() {
    say_hello("Risto");
    say_hello2("Markus");
}


