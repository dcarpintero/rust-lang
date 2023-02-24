#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 70,
    };

    println!(
        "The area of the rectangle {:?} is {} square pixels.",
        rect,
        area(&rect)
    );
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}