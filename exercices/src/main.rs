

fn main() {

    struct User {
        name: String,
        pass: String
    }

    fn create_user_struct(name: String, pass: String) -> User{
 
        let user = User {
            name,
            pass
        };

        user
        
    }


    let user1 = create_user_struct(String::from("Test"), String::from("Test"));

    println!("{}", user1.name);
}


// fn fatorial(n: i32) -> i32{
//     if n == 1 {
//         return 1
//     }

//     fatorial(n-1)*n
// }

// fn ownership_copy(n : i32) -> i32 {
//     println!("{}", n);
//     n
// } 

// fn ownership_move(s : String) -> String {
//     println!("{}", s);
//     s
// } 

