/*
    Вычисление площади прямоугольника
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    /*
    let width1: u32 = 30;
    let height1: u32 = 30;
    */

    /*
    let rect1: (u32, u32) = (30, 50);
    */

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    let scale = 2;
    let rect2 = Rectangle {
        width: 10,
        height: dbg!(scale * 33),
    };

    dbg!(&rect2);
}

/*
fn area(width: u32, height: u32) -> u32 {
    width * height
}
*/

/*
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
*/

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
