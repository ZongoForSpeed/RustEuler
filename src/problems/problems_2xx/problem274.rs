use crate::maths::primes::crible235;
use crate::register_problem;

register_problem!(
    274,
    "Divisibility Multipliers",
    problem274
);

/// For each integer p > 1 coprime to 10 there is a positive divisibility multiplier m < p which preserves
/// divisibility by p for the following function on any positive integer, n:
///
/// f(n) = (all but the last digit of n) + (the last digit of n) * m
///
/// That is, if m is the divisibility multiplier for p, then f(n) is divisible by p if and only if n is divisible by
/// p.
///
/// (When n is much larger than p, f(n) will be less than n and repeated application of f provides a multiplicative
/// divisibility test for p.)
///
/// For example, the divisibility multiplier for 113 is 34.
///
/// f(76275) = 7627 + 5 * 34 = 7797 : 76275 and 7797 are both divisible by 113
/// f(12345) = 1234 + 5 * 34 = 1404 : 12345 and 1404 are both not divisible by 113
///
/// The sum of the divisibility multipliers for the primes that are coprime to 10 and less than 1000 is 39517. What
/// is the sum of the divisibility multipliers for the primes that are coprime to 10 and less than 10**7?
pub fn problem274() -> String {
    let n = 10_000_000;

    let mut resultat = 0;
    crible235::<u64, _>(n, |p| {
        // crible235 emits primes up to the next multiple of 30 above n, so filter them out here
        if p >= n as u64 {
            return;
        }
        for k in 0..10 {
            if p * k % 10 == 9 {
                resultat += (p * k + 1) / 10;
                break;
            }
        }
    });

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem274() {
        assert_eq!(problem274(), "1601912348822");
    }
}
