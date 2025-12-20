use crate::maths::digits::palindrome;
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
        if palindrome(n, 10) && palindrome(n, 2) {
            result += n;
        }
    }
    result.to_string()
}
