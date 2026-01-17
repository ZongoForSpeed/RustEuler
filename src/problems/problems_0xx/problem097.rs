use crate::maths::power::Power;
use crate::register_problem;

register_problem!(97, "Large non-Mersenne prime", problem097);

pub fn problem097() -> String {
    // The first known prime found to exceed one million digits was discovered in 1999, and is
    // a Mersenne prime of the form 2^6972593−1; it contains exactly 2,098,960 digits.
    // Subsequently other Mersenne primes, of the form 2p−1, have been found which contain more
    // digits.
    //
    // However, in 2004 there was found a massive non-Mersenne prime which contains 2,357,207
    // digits: 28433×2^7830457+1.
    //
    // Find the last ten digits of this prime number.
    let mask: u128 = 10u128.pow(10);
    let mut mersenne = u128::power_mod(2, 7830457, mask);
    mersenne = (mersenne * 28433 + 1) % mask;
    mersenne.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem097() {
        let result = problem097();
        assert_eq!(result, "8739992577");
    }
}
