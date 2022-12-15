
struct Rectangle {
    lenght: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.lenght * self.width
    }
}

fn main() {
     let rect1 = Rectangle { 
        lenght: 50, 
        width: 30
    };

    println!("Area: {}", rect1.area());


}

fn rectangle_area(rectangle: &Rectangle) -> u32 {
    rectangle.lenght * rectangle.width
}



 

