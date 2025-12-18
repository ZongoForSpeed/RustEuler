use crate::maths::primes::crible2;
use crate::maths::timer::ScopeTimer;
use std::collections::HashSet;

pub fn problem037() -> u64 {
    let _timer = ScopeTimer::new("Problem 37 Truncatable primes", false);
    // The number 3797 has an interesting property. Being prime itself, it is possible to continuously
    // remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7.
    // Similarly we can work from right to left: 3797, 379, 37, and 3.
    //
    // Find the sum of the only eleven primes that are both truncatable from left to right and right to left.
    //
    // NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.

    let mut primes: HashSet<u64> = HashSet::new();
    crible2(1000000, |p| {
        primes.insert(p);
    });
    let mut resultat = 0;
    for &p in &primes {
        let mut q = p;
        let mut test = true;
        while q != 0 && test {
            test = primes.contains(&q);
            q /= 10;
        }
        q = 10;
        while q < p && test {
            test = primes.contains(&(p % q));
            q *= 10;
        }
        if test && p > 10 {
            resultat += p;
        }
    }

    resultat
}
