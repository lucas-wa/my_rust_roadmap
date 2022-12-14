
struct Rectangle {
    lenght: u32,
    width: u32
}

fn main() {
     let rect1 = Rectangle { 
        lenght: 50, 
        width: 30
    };

    println!("Area: {}", rectangle_area(&rect1));


}

fn rectangle_area(rectangle: &Rectangle) -> u32 {
    rectangle.lenght * rectangle.width
}



 

