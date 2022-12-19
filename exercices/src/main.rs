#[derive(Debug)]

struct Rectangle {
    lenght: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.lenght * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.lenght > other.lenght
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { lenght: size, width: size }
    }
}

fn main() {
    let square1 = Rectangle::square(5);

    println!("{:#?}", square1);


}

// fn rectangle_area(rectangle: &Rectangle) -> u32 {
//     rectangle.lenght * rectangle.width
// }



 

