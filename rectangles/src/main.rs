#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };

    println!("Area of {:?} is {}", rect1, rect1.area());

    let mut rect = Rectangle {
        width: 0,
        height: 0,
    };
    rect.set_width(1);
    println!("Updated rect: {:?}", rect);

    println!("Square with size 10: {:?}", Rectangle::square(10));
}
