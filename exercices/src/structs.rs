

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



 

