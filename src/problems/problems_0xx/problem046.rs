use crate::maths::primes::crible235;
use crate::register_problem;
use std::collections::HashSet;

register_problem!(46, "Goldbach's other conjecture", problem046);

pub fn problem046() -> String {
    // It was proposed by Christian Goldbach that every odd composite number can be written as the
    // sum of a prime and twice a square.
    //
    // 9 = 7 + 2×1²
    // 15 = 7 + 2×2²
    // 21 = 3 + 2×3²
    // 25 = 7 + 2×3²
    // 27 = 19 + 2×2²
    // 33 = 31 + 2×1²
    //
    // It turns out that the conjecture was false.
    //
    // What is the smallest odd composite that cannot be written as the sum of a prime and twice a square?
    let mut primes: HashSet<i64> = HashSet::new();
    crible235(10000, |p| {
        primes.insert(p);
    });
    for n in (9..).step_by(2) {
        if !primes.contains(&n) {
            let mut success = false;
            for i in 1.. {
                if 2 * i * i >= n {
                    break;
                }
                let p = n - 2 * i * i;
                if primes.contains(&p) {
                    success = true;
                    break;
                }
            }
            if !success {
                return n.to_string();
            }
        }
    }
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem046() {
        let result = problem046();
        assert_eq!(result, "5777");
    }
}