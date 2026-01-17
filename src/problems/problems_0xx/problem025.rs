use crate::maths::fibonacci::fibonacci;
use crate::register_problem;
use crate::utils::mpz_number::MpzNumber;

register_problem!(25, "1000-digit Fibonacci number", problem025);

pub fn problem025() -> String {
    // The Fibonacci sequence is defined by the recurrence relation:
    //
    // Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
    // Hence the first 12 terms will be:
    //
    // F1 = 1
    // F2 = 1
    // F3 = 2
    // F4 = 3
    // F5 = 5
    // F6 = 8
    // F7 = 13
    // F8 = 21
    // F9 = 34
    // F10 = 55
    // F11 = 89
    // F12 = 144
    // The 12th term, F12, is the first term to contain three digits.
    //
    // What is the index of the first term in the Fibonacci sequence to contain 1000 digits?
    for (n, f) in fibonacci::<MpzNumber>().enumerate() {
        if f.number_digits(10) >= 1000 {
            return n.to_string();
        }
    }
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem025() {
        let result = problem025();
        assert_eq!(result, "4782");
    }
}