#[cfg(test)]
mod hackerrank;

// main.rs

fn main() {
    mutability_example();
    shadowing_example();
    destructuring_example();
}

// 🔹 Розділ 3 — Mutability
fn mutability_example() {
    let mut x = 5;
    x = 6;
    println!("Mutability: x = {}", x);
}

// 🔹 Розділ 4 — Shadowing
fn shadowing_example() {
    let x = 5;
    let x = x + 1; // shadowing — перевизначення змінної
    println!("Shadowing: x = {}", x);
}

// 🔹 Розділ 6 — Destructuring
fn destructuring_example() {
    let (a, b) = (1, 2); // розпаковка tuple
    println!("Destructuring: a = {}, b = {}", a, b);
}
