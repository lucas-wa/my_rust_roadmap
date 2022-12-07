fn main() {

    let s = String::from("Texto");

    ownership_move(s);


    let n = 42;

    ownership_copy(n);

}


fn fatorial(n: i32) -> i32{
    if n == 1 {
        return 1
    }

    fatorial(n-1)*n
}

fn ownership_copy(n : i32) -> i32 {
    println!("{}", n);
    n
} 

fn ownership_move(s : String) -> String {
    println!("{}", s);
    s
} 