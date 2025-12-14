use crate::maths::premiers::crible235;
use crate::maths::timer::ScopeTimer;
use std::collections::HashSet;

pub fn problem050() -> u64 {
    let _timer = ScopeTimer::new("Problem 50 Consecutive prime sum", false);
    // The prime 41, can be written as the sum of six consecutive primes:
    //                                      41 = 2 + 3 + 5 + 7 + 11 + 13
    //
    // This is the longest sum of consecutive primes that adds to a prime below one-hundred.
    //
    // The longest sum of consecutive primes below one-thousand that adds to a prime, contains 21 terms,
    // and is equal to 953.
    //
    // Which prime, below one-million, can be written as the sum of the most consecutive primes?
    let limit: u64 = 1000000;
    let mut primes: Vec<u64> = Vec::new();
    crible235(limit as usize, |p| primes.push(p));

    let prime_set: HashSet<u64> = HashSet::from_iter(primes.iter().cloned());
    let mut result = 0;
    let mut length = 0;

    let size = primes.len();
    for i in 0..size {
        let mut s = primes[i];
        let mut l = 1;
        for j in i + 1..size {
            s += primes[j];
            l += 1;
            if s > limit {
                break;
            }
            if l > length && prime_set.contains(&s) {
                length = l;
                result = s;
            }
        }
    }

    result
}
