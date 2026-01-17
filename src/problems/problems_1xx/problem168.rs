use crate::register_problem;
use crate::utils::mpz_number::MpzNumber;
use itertools::iproduct;
use num_traits::Zero;

register_problem!(168, "Number Rotations", problem168);

pub fn problem168() -> String {
    // Consider the number 142857. We can right-rotate this number by moving the last digit (7) to the front of it,
    // giving us 714285.
    // It can be verified that 714285=5×142857.
    // This demonstrates an unusual property of 142857: it is a divisor of its right-rotation.
    //
    // Find the last 5 digits of the sum of all integers n, 10 < n < 10^100, that have this property.
    let mut result = MpzNumber::new();

    let zero = MpzNumber::zero();

    for (power, a, n) in iproduct!(0..=100, 1..10, 1..10) {
        let numer: MpzNumber = a * (MpzNumber::power_ui(10, power) - n);
        let denom = MpzNumber::from(10 * n - 1);

        if (&numer % &denom) == zero {
            let mut valeur: MpzNumber = numer / denom;
            if valeur.number_digits(10) == power {
                valeur = 10 * valeur + a;
                if valeur.number_digits(10) <= 100 {
                    result += valeur;
                }
            }
        }
    }

    result %= 100000;
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem168() {
        let result = problem168();
        assert_eq!(result, "59206");
    }
}
