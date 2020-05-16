fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 30
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 20,
        height: 15,
    };

    println!(
        "The area of the rectangle is {} sq pixels",
        rect1.area()
    );
    println!("rect1 is {:?}", rect1);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // self, kinda like python hmmm
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /*
    This is an associated function
    Associated functions do not have a self, kinda analogous to class function
    Call them using ::, so Rectangle::square(1)
    fn create_square(l: u32) -> Rectangle {
        Rectangle {
                width: size,
                height: size,
            }
        }
    }
    */
}
