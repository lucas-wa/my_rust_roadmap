fn main(){

}

fn ownership_copy(n : i32) -> i32 {
    println!("{}", n);
    n
} 

fn ownership_move(s : String) -> String {
    println!("{}", s);
    s
}