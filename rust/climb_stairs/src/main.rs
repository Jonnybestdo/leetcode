fn climb_stairs(n: i32) -> i32 {
    if n <= 3 {
        return n;
    }
    let mut p1 = 2;
    let mut p2 = 3;
    let mut s = 0;
    for _ in 3..n {
        s = p1 + p2;
        p1 = p2;
        p2 = s;
    }
    s    
}

fn main() {
    println!("Hello, world!");
    println!("ways: {}", climb_stairs(45));
}
