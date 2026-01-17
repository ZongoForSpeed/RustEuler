use crate::maths::digits::Digits;
use crate::register_problem;

register_problem!(36, "Double-base palindromes", problem036);

pub fn problem036() -> String {
    // The decimal number, 585 = 1001001001_2 (binary), is palindromic in both bases.
    //
    // Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
    //
    // (Please note that the palindromic number, in either base, may not include leading zeros.)
    let mut result = 0;
    for n in 1..1000000 {
        if n.palindrome(10) && n.palindrome(2) {
            result += n;
        }
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem036() {
        let result = problem036();
        assert_eq!(result, "872187");
    }
}