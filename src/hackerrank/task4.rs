pub fn grading_students(grades: Vec<i32>) -> Vec<i32> {
    grades.into_iter().map(|g| {
        if g >= 38 && g % 5 >= 3 {
            g + (5 - g % 5)
        } else {
            g
        }
    }).collect()
}

fn main() {
    let grades = vec![73, 67, 38, 33];
    let result = grading_students(grades);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(grading_students(vec![73, 67, 38, 33]), vec![75, 67, 40, 33]);
    }
}