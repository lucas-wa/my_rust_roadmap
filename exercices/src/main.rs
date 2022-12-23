

fn main(){

    // Using a enum to not use a struct
    enum ipAdress {
        V4(u8, u8, u8, u8),
        V8(String),
    }

    let local = ipAdress::V4(127, 0, 0, 1); 
    
    let loopback = ipAdress::V6(String::from("::01"))
}