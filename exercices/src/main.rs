

fn main(){

    enum State {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel, 
        Dime,
        Quarter(State),
    }

    fn coin_value(coin : Coin) -> u32{
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("Quarter do estado {:?}", state);
                25   
            },
        }
    }
}