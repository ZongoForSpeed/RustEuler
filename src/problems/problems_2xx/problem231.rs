use crate::maths::primes::crible235;
use crate::register_problem;

register_problem!(
    231,
    "The prime factorisation of binomial coefficients",
    problem231
);

fn x(mut n: u64, p: u64) -> u64 {
    let mut result = 0;
    while n > 0 {
        n /= p;
        result += n;
    }
    result
}

pub fn problem231() -> String {
    // The binomial coefficient 10C3 = 120. 120 = 23 × 3 × 5 = 2 × 2 × 2 × 3 × 5, and 2 + 2 + 2 + 3 + 5 = 14.
    // So the sum of the terms in the prime factorisation of 10C3 is 14.
    //
    // Find the sum of the terms in the prime factorisation of 20000000C15000000.
    let n = 20000000;
    let k = 15000000;

    let mut primes = Vec::new();
    crible235(n as usize, |p| primes.push(p));

    let mut result = 0;
    for p in primes {
        result += p * (x(n, p) - x(k, p) - x(n - k, p));
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem231::problem231;

    #[test]
    fn test_problem231() {
        let result = problem231();
        assert_eq!(result, "7526965179680");
    }
}
