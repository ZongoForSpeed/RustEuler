use crate::maths::digits::Digits;
use crate::register_problem;

register_problem!(4, "Largest palindrome product", problem004);

pub fn problem004() -> String {
    // A palindromic number reads the same both ways. The largest palindrome made from the product
    // of two 2-digit numbers is 9009 = 91 × 99.
    //
    // Find the largest palindrome made from the product of two 3-digit numbers.
    let mut solution = 0;
    for a in 100..1000 {
        for b in 100..a {
            let ab = a * b;
            if ab.palindrome(10) {
                solution = u64::max(solution, ab);
            }
        }
    }
    solution.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem004() {
        let result = problem004();
        assert_eq!(result, "906609");
    }
}
