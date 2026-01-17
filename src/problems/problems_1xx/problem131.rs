use crate::maths::primes::crible235;
use crate::register_problem;
use num_integer::Roots;
use std::collections::HashSet;

register_problem!(131, "Prime cube partnership", problem131);

pub fn problem131() -> String {
    // There are some prime values, p, for which there exists a positive integer, n, such that the expression
    // n^3 + n^2p is a perfect cube.
    //
    // For example, when p = 19, 8^3 + 8^2×19 = 12^3.
    //
    // What is perhaps most surprising is that for each prime with this property the value of n is unique, and there are
    // only four such primes below one-hundred.
    //
    // How many primes below one million have this remarkable property?
    let limit = 1000000;
    let mut primes: HashSet<usize> = HashSet::new();
    crible235(limit, |p| {
        primes.insert(p);
    });

    let mut result = 0;
    for a in 1..(limit / 3).sqrt() {
        let b = a + 1;
        let p = b * b * b - a * a * a;
        if p < limit && primes.contains(&p) {
            result += 1;
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem131() {
        let result = problem131();
        assert_eq!(result, "173");
    }
}
