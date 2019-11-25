use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::{stdin, Read};

fn main() {
    let rand_array = [1,2,3];
    println!("{}", rand_array[2]);
    println!("{}", rand_array.len());
    println!("Second: {:?}", &rand_array[1..3]);
    let mut vect1 = vec![1,2,3,4,5];
    println!("{}", vect1[1]);
    for i in &vect1 {
        println!("{}", i);
    }
    vect1.push(6);
    vect1.pop();

    let rand_tuple = ("Derek", 40);

    let rand_tuple_2: (&str, i8) = ("Derek", 40);

    println!("{}", rand_tuple_2.0)


}