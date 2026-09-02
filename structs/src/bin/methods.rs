struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // short for self: &Self
        self.width * self.height
    }

    fn width(&self) -> bool { // pode ter o nome de um campo
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self { // função associada não recebe self como param
        Self { // Self é um alias pra Rectangle aqui e na linha anterior
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
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The rect area = {}",
        rect1.area()
    );

    if rect1.width() {
        println!("Non zero width: {}", rect1.width)
    };

    println!("rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq_rect = Rectangle::square(3); // funcoes associadas são chamadas de forma diferente
    println!("width: {}, height: {}, area: {}", sq_rect.width, sq_rect.height, sq_rect.area());
}