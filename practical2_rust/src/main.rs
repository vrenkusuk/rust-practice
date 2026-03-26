fn main() {
    // Shadowing
    let x = 10;
    let x = x + 1;
    println!("x after shadowing: {}", x);

    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("Max points: {}", MAX_POINTS);

    // Scalar types
    let a: i32 = 10;
    let b: f64 = 3.14;
    let c: bool = true;
    let d: char = '🦀';
    println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);

    // Scope example
    let y: i32;
    {
        y = 5;
        println!("Inner scope value of x is {} and value of y is {}", x, y);
    }
    println!("Outer scope value of x is {} and value of y is {}", x, y);
}
