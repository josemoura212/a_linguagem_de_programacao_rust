#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn new(length: u32, width: u32) -> Self {
        Rectangle { length, width }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    fn square(size: u32) -> Self {
        Rectangle {
            length: size,
            width: size,
        }
    }
}

fn _area(retangulo: &Rectangle) -> u32 {
    retangulo.width * retangulo.length
}

pub fn rect() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };

    let rect2 = Rectangle::new(40, 10);
    let rect3 = Rectangle::new(45, 60);

    let _sq = Rectangle::square(50);

    println!("rect1 is {:#?}", rect1);

    println!("A area do retangulo e de {} pixels", rect1.area());

    println!("O rect1 cabe em rect2? {}", rect1.can_hold(&rect2));
    println!("O rect1 cabe em rect3? {}", rect1.can_hold(&rect3));
}
