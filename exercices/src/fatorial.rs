fn main () {
    fatorial(3);
}

fn fatorial(n: i32) -> i32{
    if n == 1 {
        return 1
    }

    fatorial(n-1)*n
}