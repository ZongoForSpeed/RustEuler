use crate::maths::arithmetic::Arithmetic;
use crate::maths::primes::{Primes, crible235};
use crate::register_problem;
use crate::utils::timer::ScopeTimer;

register_problem!(245, "Coresilience", problem245);

fn is_prime(n: u64, primes: &Vec<u64>) -> bool {
    if n < 100 {
        return match n {
            2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 | 23 | 29 | 31 | 37 | 41 | 43 | 47 | 53 | 59 | 61
            | 67 | 71 | 73 | 79 | 83 | 89 | 97 => true,
            _ => false,
        };
    }

    if n < primes[primes.len() - 1] {
        return match primes.binary_search(&n) {
            Ok(_) => true,
            Err(_) => false,
        };
    }

    n.miller_rabin(15)
}

fn find(
    primes: &Vec<u64>,
    limit: u64,
    n: u64,
    phi_n: u64,
    primes_left: usize,
    min_index: usize,
) -> u64 {
    let mut result = 0;
    if primes_left == 1 {
        let min_p = primes[min_index];
        let min_k = ((n * min_p - 1) / (min_p * (n - phi_n) + phi_n)) + 1;
        let max_k = n / (n - phi_n);

        for k in min_k..=max_k {
            let numer = k * phi_n + 1;
            let denom = n - k * (n - phi_n);

            if denom == 0 {
                break;
            }

            if numer % denom == 0 {
                let candidate = numer / denom;

                if candidate * n > limit {
                    break;
                }

                if is_prime(candidate, primes) {
                    result += n * candidate;
                }
            }
        }
    } else {
        for index in min_index + 1..primes.len() {
            let mut borne = n;
            for i in 0..primes_left {
                borne *= primes[index + i];
            }

            if borne > limit {
                break;
            }
            let next_n = n * primes[index];
            let next_phi_n = phi_n * (primes[index] - 1);

            if u64::gcd(next_n, next_phi_n) == 1 {
                result += find(&primes, limit, next_n, next_phi_n, primes_left - 1, index);
            }
        }
    }

    result
}

pub fn problem245() -> String {
    // We shall call a fraction that cannot be cancelled down a resilient fraction.
    // Furthermore we shall define the resilience of a denominator, R(d), to be the ratio of its
    // proper fractions that are resilient; for example, R(12) = 4/11.
    //
    // The resilience of a number d > 1 is then	φ(d) / d − 1, where φ is Euler's totient function.
    // We further define the coresilience of a number n > 1 as C(n)	= (n − φ(n)) / (n − 1)
    //
    // The coresilience of a prime p is C(p) = 1 / (p - 1)
    //
    // Find the sum of all composite integers 1 < n ≤ 2×1011, for which C(n) is a unit fraction.
    let limit = 200000000000;

    let mut primes = Vec::new();
    crible235(1_000_000_000, |p| primes.push(p));

    let mut result = 0;
    for i in 2.. {
        let label = format!("Find composite for i={}", i);
        let _timer = ScopeTimer::new(label.as_str(), false);

        let somme = find(&primes, limit, 1, 1, i, 0);
        println!(
            "Somme nombre composite produit de {} premiers = {}",
            i, somme
        );
        if somme == 0 {
            break;
        }
        result += somme;
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem245::problem245;

    #[test]
    fn test_problem245() {
        let result = problem245();
        assert_eq!(result, "288084712410001");
    }
}
