use crate::maths::digits::Digits;
use crate::maths::power::Power;
use crate::maths::primes::crible235;
use crate::register_problem;

register_problem!(134, "Prime pair connection", problem134);

pub fn problem134() -> String {
    // Consider the consecutive primes p1 = 19 and p2 = 23. It can be verified that 1219 is the smallest number such
    // that the last digits are formed by p1 whilst also being divisible by p2.
    //
    // In fact, with the exception of p1 = 3 and p2 = 5, for every pair of consecutive primes, p2 > p1, there exist
    // values of n for which the last digits are formed by p1 and n is divisible by p2. Let S be the smallest of these
    // values of n.
    //
    // Find ∑ S for every pair of consecutive primes with 5 ≤ p1 ≤ 1000000.
    let limit: u64 = 1000000;
    let mut primes: Vec<u64> = Vec::new();
    crible235(limit as usize, |p| primes.push(p));

    let mut result = 0;
    for x in primes.windows(2) {
        let p1 = x[0];
        let p2 = x[1];
        if p1 > 4 && p1 < limit {
            let d1 = p1.count_digits(10) as u64;
            let mut b = u64::power_mod(10, d1 * (p2 - 2), p2);
            b *= p2 - p1;
            b %= p2;
            result += b * u64::power(10, d1) + p1;
        }
    }
    
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem134() {
        let result = problem134();
        assert_eq!(result, "18613426663617118");
    }
}
