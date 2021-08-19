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

impl<'a> Rectangle {
    fn to_string(&self, extra: &'a str) -> &'a str {
        extra
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };
    println!("rect {:?} 's area is {}", rect1, area(&rect1));
    println!("area is {}", rect1.area());
    let rect2 = Rectangle::square(12);
    println!(
        "{:?} can hold {:?}: {}",
        rect1,
        rect2,
        rect1.can_hold(&rect2)
    );
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
