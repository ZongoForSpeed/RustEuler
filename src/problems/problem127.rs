use crate::maths::arithmetic::Arithmetic;
use crate::maths::primes::crible23;
use crate::register_problem;
use std::collections::BTreeSet;

register_problem!(127, "abc-hits", problem127);

pub fn problem127() -> String {
    // The radical of n, rad(n), is the product of distinct prime factors of n. For example,
    // 504 = 23 × 32 × 7, so rad(504) = 2 × 3 × 7 = 42.
    //
    // We shall define the triplet of positive integers (a, b, c) to be an abc-hit if:
    //
    //      GCD(a, b) = GCD(a, c) = GCD(b, c) = 1
    //      a < b
    //      a + b = c
    //      rad(abc) < c
    //
    // For example, (5, 27, 32) is an abc-hit, because:
    //
    //      GCD(5, 27) = GCD(5, 32) = GCD(27, 32) = 1
    //      5 < 27
    //      5 + 27 = 32
    //      rad(4320) = 30 < 32
    //
    // It turns out that abc-hits are quite rare and there are only thirty-one abc-hits for c < 1000,
    // with ∑c = 12523.
    //
    // Find ∑c for c < 120000.
    let limit = 120000;
    let mut primes = BTreeSet::new();
    crible23(limit, |p| {
        primes.insert(p);
    });

    let mut radical = vec![0, 1];
    for n in 2..limit {
        radical.push(n.radical(&primes));
    }

    let mut result = 0;
    for c in 3..limit {
        if primes.contains(&c) {
            continue;
        }

        let rad_c = radical[c];
        if rad_c < c {
            for a in 1..c / 2 {
                let b = c - a;
                let rad_a = radical[a];
                let rad_b = radical[b];

                if rad_a * rad_b * rad_c < c && usize::gcd(a, b) == 1 {
                    result += c;
                }
            }
        }
    }

    result.to_string()
}
