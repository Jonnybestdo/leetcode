fn climb_stairs(n: i32) -> i32 {
    let mut p = 1;
    let mut w = 1;
    let mut t;
    for _ in 1..n {
        t = p;
        p = w;
        w = t;
        w += p;
    }
    w    
}

fn main() {
    println!("Hello, world!");
    println!("ways: {}", climb_stairs(45));
}
