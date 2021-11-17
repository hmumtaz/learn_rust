#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods
impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    fn is_square(&self) -> bool {
        return self.width == self.height;
    }
}

// Functions
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        return Rectangle {width: size, height: size};
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };



    println!("rect1 is {:?}", rect1);
    println!("The area of the rect1 is {} square pixels", rect1.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(20);

    println!("Is rect4 square? {}", rect4.is_square());
}
