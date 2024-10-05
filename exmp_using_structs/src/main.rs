#[derive(Debug)]
struct  Rectangle {
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
    
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

let scale = 2;

    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("rect1 is {rect1:?}");
    // let rect = (30, 50);

    println!("rect1 is {rect:?}");

print!("The area of the rectangle is {} square pixels.", rect.area());
// let's try to use the can_hold method
let rect2 = Rectangle {
    width: 10,
    height: 40,
};

println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );

    println!("Hello, world!");
}

fn area(dimensions: &Rectangle) -> u32 {
 dimensions.width * dimensions.height
}
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }
