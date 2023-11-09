fn area1(width: i32, height: i32) -> i32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );

    let rect2 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect2)
    );

    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect3)
    );

    println!("rect3 is {:?}", rect3);

    dbg!(&rect3);
}
