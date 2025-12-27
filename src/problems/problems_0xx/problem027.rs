use std::collections::HashSet;
use num_traits::abs;
use crate::maths::primes::crible235;
use crate::register_problem;

fn prime_seq(primes: &HashSet<i64>, a: i64, b: i64) -> i64 {
    let mut compteur = 0;
    let mut n = b;

    while primes.contains(&n) {
        compteur += 1;
        n = compteur * compteur + a * compteur + b;
    }

    compteur
}

register_problem!(27, "Quadratic primes", problem027);

pub fn problem027() -> String {
    // Euler discovered the remarkable quadratic formula:
    //
    //                                                n² + n + 41
    //
    // It turns out that the formula will produce 40 primes for the consecutive values n = 0 to 39.
    // However, when n = 40, 402 + 40 + 41 = 40(40 + 1) + 41 is divisible by 41, and certainly when n = 41,
    // 41² + 41 + 41 is clearly divisible by 41.
    //
    // The incredible formula  n² − 79n + 1601 was discovered, which produces 80 primes for the consecutive values n = 0
    // to 79. The product of the coefficients, −79 and 1601, is −126479.
    //
    // Considering quadratics of the form:
    //
    // n² + an + b, where |a| < 1000 and |b| < 1000
    //
    // where |n| is the modulus/absolute value of n
    // e.g. |11| = 11 and |−4| = 4
    // Find the product of the coefficients, a and b, for the quadratic expression that produces the maximum number of
    // primes for consecutive values of n, starting with n = 0.
    let mut primes: HashSet<i64> = HashSet::new();
    crible235(1000000, |p| { primes.insert(p);} );

    let mut a_max = 0;
    let mut b_max = 0;
    let mut c_max = 0;
    for b in 2..1000 {
        if primes.contains(&b) {
            for a in -999..1000 {
                if abs(a) > 1 {
                    let compteur = prime_seq(&primes, a, b);
                    if compteur > c_max {
                        c_max = compteur;
                        a_max = a;
                        b_max = b;
                        // std::cout << "(a, b, c) = (" << a << ", " << b << ", " << compteur << ")" << std::endl;
                    }
                }

            }
        }
    }
    let result = a_max * b_max;
    result.to_string()
}
