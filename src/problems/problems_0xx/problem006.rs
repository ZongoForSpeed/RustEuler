use crate::register_problem;

register_problem!(6, "Sum square difference", problem006);

pub fn problem006() -> String {
    // The sum of the squares of the first ten natural numbers is, 1² + 2² + ... + 10² = 385
    // The square of the sum of the first ten natural numbers is, (1 + 2 + ... + 10)² = 55² = 3025
    // Hence the difference between the sum of the squares of the first ten natural numbers and the
    // square of the sum is 3025 − 385 = 2640.
    //
    // Find the difference between the sum of the squares of the first one hundred natural numbers
    // and the square of the sum.
    let mut somme: u64 = 0;
    let mut somme_carre: u64 = 0;
    for n in 1..=100 {
        somme += n;
        somme_carre += n * n;
    }
    let solution = somme * somme - somme_carre;
    solution.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem006() {
        let result = problem006();
        assert_eq!(result, "25164150");
    }
}
