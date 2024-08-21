fn main() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle { width: 30, height: 50 };
    println!("Rectangle width: {}, Rectangle height: {}", rect1.width, rect1.height);
}
