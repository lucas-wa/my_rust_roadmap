
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
}

fn main() {
     let rect1 = Rectangle { 
        lenght: 50, 
        width: 30
    };

    let rect2 = Rectangle { 
        lenght: 20, 
        width: 10
    };

    println!("{}", rect1.can_hold(&rect2));


}

// fn rectangle_area(rectangle: &Rectangle) -> u32 {
//     rectangle.lenght * rectangle.width
// }



 

