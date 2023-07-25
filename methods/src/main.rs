#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // first param is always self (can also be mut)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // methods can be named like fields and return something else
    fn witdth(&self) -> bool {
        self.width > 0
    }

    // ... or can just be getters
    fn height(&self) -> u32 {
        self.height
    }

    // Not a method but an associated function
    // ...it does not take self as first param
    // often used as constructors
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let square = Rectangle::square(10);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Width is > 0?  {}.", rect1.witdth());
    println!("height is  {}.", rect1.height());
    println!("square size  {} x {}.", square.width, square.height);
}
