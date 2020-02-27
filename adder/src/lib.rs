#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn add_three(a: i32) -> i32 {
    a + 3
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100, got")]
    fn greater_than_100() {
        Guess::new(201);
    }

    #[test]
    #[ignore]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"),
                "Greeting did not contain name, value was `{}`", result);
    }

    #[test]
    fn it_adds_two() {
        // This can be simplified
        assert!(add_two(2) == 4);

        // Like this
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn test_add_three() {
        assert_ne!(4, add_three(2));
    }


    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
