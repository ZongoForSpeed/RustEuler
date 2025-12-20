use crate::maths::digits::loop_digits;
use crate::maths::power;
use crate::register_problem;

register_problem!(30, "Digit fifth powers", problem030);

pub fn problem030() -> String {
    // Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:
    //
    // 1634 = 1^4 + 6^4 + 3^4 + 4^4
    // 8208 = 8^4 + 2^4 + 0^4 + 8^4
    // 9474 = 9^4 + 4^4 + 7^4 + 4^4
    // As 1 = 1^4 is not a sum it is not included.
    //
    // The sum of these numbers is 1634 + 8208 + 9474 = 19316.
    //
    // Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.
    let mut result = 0;
    for n in 2..200000 {
        let mut s = 0;
        loop_digits(n, 10, |d| s += power::power(d, 5));
        if s == n {
            result += n;
        }
    }

    result.to_string()
}
