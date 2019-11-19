fn sqr(x: f64) -> f64 {
    x * x
}

fn abs (x: f64) -> f64 {
    if x > 0.0 {
        x
    }
    else { -x }
}

fn clamp(x: f64, x1: f64, x2: f64) -> f64 {
    if x < x1 {
        x1
    } else if x > x2 {
        x2
    } else {
        x
    }
}

fn main() {
    let re = sqr(6.0);
    println!("sqr {}", re);

    let re2 = abs(-30.5);
    let re22 = abs(30.5);
    println!("{} || {}", re2, re22)

    let re3 = clamp(re, re2, re22);
    print("{}", re3)
//    for i in 0..5 {
//        println!("Hello {}", i)
//    }
//
//    let test = "Hello";
//    for i in 0..10 {
//        let even_odd = if i % 2 == 0 {"even"} else {"odd"};
//        println!("{} : {} {}",test, even_odd, i);
//    }
//
//    let mut j = 0.0;
//    for i in 0..10  {
//        j += i as f64
//    }
//    print!("{}", j)

}