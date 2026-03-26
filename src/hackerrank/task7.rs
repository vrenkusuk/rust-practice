pub fn between_two_sets(a: Vec<i32>, b: Vec<i32>) -> i32 {
    (1..=100)
        .filter(|x| a.iter().all(|ai| x % ai == 0) && b.iter().all(|bi| bi % x == 0))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(between_two_sets(a, b), 3);
    }

    #[test]
    fn test_example2() {
        let a = vec![3, 4];
        let b = vec![24, 48];
        assert_eq!(between_two_sets(a, b), 2);
    }
}