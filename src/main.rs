// main.rs

fn main() {
    // Приклад змінної, яку можна змінювати
    let mut score = 70;
    println!("Початковий бал: {}", score);

    // Змінюємо бал
    score += 5;
    println!("Після бонусу: {}", score);

    // Shadowing — перевизначаємо змінну
    let score = score * 2;
    println!("Подвоєний бал: {}", score);

    // Destructuring — розпаковка кортежу
    let (passed, failed) = calculate_results(score);
    println!("Passed: {}, Failed: {}", passed, failed);
}

// Функція для демонстрації destructuring
fn calculate_results(total: i32) -> (i32, i32) {
    let passed = total / 10;
    let failed = 10 - passed;
    (passed, failed)
}