use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::{stdin, Read};

mod functions;
use functions::say_hello;

mod test;
use test::testisetti::say_hello2;


fn main() {
    say_hello("test");
    say_hello2("test2");

    let hulk = Hero::Strong(100);
    let quicksilver = Hero::Fast;
    let spideman = Hero::Info {name:"Spiderman".to_owned(), secret: "Peter Parker".to_owned()};

    get_info(hulk);
    get_info(spideman)
}

enum Hero {
    Fast,
    Strong(i32),
    Info {name: String, secret: String}
}

fn get_info(h: Hero) {
    match h {
        Hero::Fast => println!("Fast"),
        Hero::Strong(i) => println!("Lifts {} tons!", i),
        Hero::Info {name, secret} => println!("{} is {}", name, secret),
    }
}


