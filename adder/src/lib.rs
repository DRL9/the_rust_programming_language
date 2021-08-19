#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let large = Rectangle {
            width: 9,
            height: 7,
        };
        let small = Rectangle {
            width: 5,
            height: 3,
        };
        assert!(large.can_hold(&small));
    }

    // #[test]
    fn greeting_contain_name() {
        let name = "Mike";
        assert!(
            greeting(&name).contains("Mike"),
            "Greeting not contain {}",
            &name
        );
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        wrong_fn()
    }

    #[test]
    #[should_panic(expected = "wron")]
    fn test_panic2() {
        wrong_fn()
    }
}

fn wrong_fn() {
    panic!("wrong")
}

fn greeting(name: &str) -> String {
    format!("Hello ")
}

pub fn add_two(num: u32) -> u32 {
    num + 2
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
