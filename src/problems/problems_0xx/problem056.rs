use crate::register_problem;
use crate::utils::mpz_number::MpzNumber;

register_problem!(56, "Powerful digit sum", problem056);

pub fn problem056() -> String {
    // A googol (10^100) is a massive number: one followed by one-hundred zeros; 100^100 is almost
    // unimaginably large: one followed by two-hundred zeros. Despite their size, the sum of the
    // digits in each number is only 1.
    //
    // Considering natural numbers of the form, a^b, where a, b < 100, what is the maximum digital sum?
    let mut maximum = 0;
    for (a, b) in itertools::iproduct!(1..100, 1..100) {
        let sum = MpzNumber::power_ui(a, b).sum_digits(10);
        maximum = std::cmp::max(maximum, sum);
    }
    maximum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem056() {
        let result = problem056();
        assert_eq!(result, "972");
    }
}
