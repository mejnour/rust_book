#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// variaveis soltas que não compartilham vínculo algum
fn area_1(width: u32, height: u32) -> u32 {
    width * height
}

// tupla compartilha vinculo mas perde expressividade
fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// struct mantem expressividade e organização
fn area_3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "1 - The area of the rectangle is {} square pixels.",
        area_1(width1, height1)
    );

    println!(
        "2 - The area of the rectangle is {} square pixels.",
        area_2((width1, height1))
    );

    let rect1 = Rectangle {
        width: width1,
        height: height1,
    };

    println!(
        "3 - The area of the rectangle is {} square pixels.",
        area_3(&rect1)
    );

    println!("rect1 {rect1:#?}"); // só funciona com #[derive(Debug)] na struct

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);
}