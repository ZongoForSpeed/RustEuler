use crate::maths::primes::crible235;
use crate::register_problem;

register_problem!(187, "Semiprimes", problem187);

pub fn problem187() -> String {
    // A composite is a number containing at least two prime factors. For example, 15 = 3 × 5; 9 = 3 × 3;
    // 12 = 2 × 2 × 3.
    //
    // There are ten composites below thirty containing precisely two, not necessarily distinct, prime factors: 4, 6, 9,
    // 10, 14, 15, 21, 22, 25, 26.
    //
    // How many composite integers, n < 10^8, have precisely two, not necessarily distinct, prime factors?
    let limit = 100000000;
    let mut primes = Vec::new();
    crible235(limit, |p| primes.push(p));

    let mut pi = Vec::with_capacity(limit);

    let mut q = 0;
    let mut n = 0;
    for &p in &primes {
        pi.resize(pi.len() + (p - q), n);
        n += 1;
        q = p;
    }

    let mut result = 0;
    for &p in &primes {
        if p * p > limit {
            break;
        }

        result += pi[limit / p] - pi[p] + 1;
    }

    result.to_string()
}
