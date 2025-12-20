use crate::register_problem;
use crate::utils::mpz_number::MpzNumber;

register_problem!(20, "Factorial digit sum", problem020);

pub fn problem020() -> String {
    // n! means n × (n − 1) × ... × 3 × 2 × 1
    //
    // For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
    // and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
    //
    // Find the sum of the digits in the number 100!
    MpzNumber::factorial(100).sum_digits(10).to_string()
}
