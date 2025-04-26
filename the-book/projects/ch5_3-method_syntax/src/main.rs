#[derive(Clone, Copy)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    fn set_to_max(&mut self, other: Rectangle) {
        let max = self.max(other); // We can now move out of `*self` because #[derive(Clone, Copy)]
        *self = max;
    }

    // NOTE: All functions defined withing an `impl` block are called
    // 'associated functions', often used for construct the new instance.
    // Use `::` to call function; `let sq = Rectangle::square(3);`
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// Multiple `impl` Blocks
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
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

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect 1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect 1 hold rect3? {}", rect1.can_hold(&rect3));

    // Method Calls are Syntactic Sugar for Function Calls
    let mut r = Rectangle {
        width: 1,
        height: 2,
    };
    let area1 = r.area();
    let area2 = Rectangle::area(&r);
    assert_eq!(area1, area2);

    r.set_width(2);
    Rectangle::set_width(&mut r, 2);

    // Methods and Ownership
    {
        let rect = Rectangle {
            width: 0,
            height: 0,
        };

        println!("{}", rect.area());

        let other_rect = Rectangle {
            width: 1,
            height: 1,
        };
        let max_rect = rect.max(other_rect);
        // rect.set_width(0); // ERROR: `rect` was moved
    }

    {
        let rect = Rectangle {
            width: 0,
            height: 0,
        };
        // rect.set_width(0); // ERROR: `rect` was immutable

        // Added the mut keyword to the let-binding
        let mut rect = Rectangle {
            width: 0,
            height: 0,
        };
        rect.set_width(1); // this is now ok

        let rect_ref = &rect;
        // rect_ref.set_width(2); // ERROR: but this is still not ok, missing W
    }
}
