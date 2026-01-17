use crate::maths::arithmetic::Arithmetic;
use crate::maths::primes::crible235;
use crate::register_problem;

register_problem!(179, "Consecutive positive divisors", problem179);

pub fn problem179() -> String {
    // Find the number of integers 1 < n < 107, for which n and n + 1 have the same number of positive divisors.
    // For example, 14 has the positive divisors 1, 2, 7, 14 while 15 has 1, 3, 5, 15.
    let limit = 10000000;
    let mut primes = Vec::new();
    crible235(limit, |p| primes.push(p));

    let mut result = 0;
    let mut sigma = 1;

    for n in 2..limit {
        let s = n.number_of_divisors(&primes);
        if sigma == s {
            result += 1;
        } else {
            sigma = s;
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem179() {
        let result = problem179();
        assert_eq!(result, "986262");
    }
}
