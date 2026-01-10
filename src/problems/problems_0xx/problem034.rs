use crate::maths::digits::Digits;
use crate::maths::factorial::Factorial;
use crate::register_problem;

register_problem!(34, "Digit factorials", problem034);

pub fn problem034() -> String {
    // 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
    //
    // Find the sum of all numbers which are equal to the sum of the factorial of their digits.
    //
    // Note: as 1! = 1 and 2! = 2 are not sums they are not included.
    let borne: u64 = 2540160; // 7*9!
    let mut solution: u64 = 0;
    for n in 10..borne {
        let mut s = 0;
        n.loop_digits( 10, |d| s += u64::factorial(d));
        if s == n {
            solution += n;
        }
    }
    solution.to_string()
}
