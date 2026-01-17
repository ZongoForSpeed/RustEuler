use crate::register_problem;
use crate::utils::mpz_number::MpzNumber;

register_problem!(16, "Power digit sum", problem016);

pub fn problem016() -> String {
    // 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
    //
    // What is the sum of the digits of the number 2^1000?
    let result = MpzNumber::power_ui(2, 1000).sum_digits(10);
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem016() {
        let result = problem016();
        assert_eq!(result, "1366");
    }
}