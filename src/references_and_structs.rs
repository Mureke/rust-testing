use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::{stdin, Read};

fn main() {
    let prim_val = 1;
    let prim_val2 = prim_val;

    println!("prim_val: {}", prim_val);

    let vect2 = vec![1, 2, 3];
    println!("Sum of vect: {}", sum_vects(&vect2));
    println!("Vect: {:?}", vect2);

    let mut circle1 = Circle {
        x: 10.0,
        y: 10.0,
        radius: 10.0,
    };

    println!("X: {}, Y:{}, Radius: {}", circle1.x, circle1.y, circle1.radius);

    println!("Circle Radius: {}", get_radius(&circle1));

    println!("Circle x: {}", circle1.get_x());

    println!("Circle area: {}", circle1.area());

    let mut  rectangle1 = Rectangle{
        width: 10.0, height: 10.0,
    };

    println!("Rectangle area: {}", rectangle1.area());

}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

fn get_radius(circle: &Circle) -> f64 {
    circle.radius
}

impl Circle {
    pub fn get_x(&self) -> f64 {
        self.x
    }
}

struct Rectangle {
    height: f64,
    width: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        3.14159 * (self.radius * self.radius)
    }
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }
}

fn sum_vects(v1: &Vec<i32>) -> i32 {
    let sum = v1.iter().fold(0, |mut sum, &x| {
        sum += x;
        sum
    });
    sum
}
