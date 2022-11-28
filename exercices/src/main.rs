fn main() {

    let res = fatorial(5);

    println!("{}", res);
}


fn fatorial(n: i32) -> i32{
    if n == 1 {
        return 1
    }

    fatorial(n-1)*n
}