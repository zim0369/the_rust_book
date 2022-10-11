// ANCHOR: all
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // ANCHOR: def_width
    fn width(&self) -> bool {
        self.width > 0
    }
    // ANCHOR_END: def_width

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width < self.width && other.height < self.height
    }
}

// ANCHOR: associated_function
impl Rectangle {
    // Associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
}
// ANCHOR_END: associated_function

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle { ..rect1 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // ANCHOR: call_width
    if rect1.width() {
        dbg!(rect1.width);
    }
    // ANCHOR_END: call_width

    dbg!(rect1.can_hold(&rect2));

    // ANCHOR: call_associated_function
    dbg!(Rectangle::square(5));
    // ANCHOR_END: call_associated_function
}
// ANCHOR_END: all
