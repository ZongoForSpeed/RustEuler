use crate::maths::arithmetic::Arithmetic;
use crate::maths::primes::crible235;
use crate::register_problem;
use std::collections::HashSet;

register_problem!(130, "Composites with prime repunit property", problem130);

pub fn problem130() -> String {
    // A number consisting entirely of ones is called a repunit. We shall define R(k) to be a repunit of length k; for
    // example, R(6) = 111111.
    //
    // Given that n is a positive integer and GCD(n, 10) = 1, it can be shown that there always exists a value, k, for
    // which R(k) is divisible by n, and let A(n) be the least such value of k; for example, A(7) = 6 and A(41) = 5.
    //
    // You are given that for all primes, p > 5, that p − 1 is divisible by A(p). For example, when p = 41, A(41) = 5,
    // and 40 is divisible by 5.
    //
    // However, there are rare composite values for which this is also true; the first five examples being 91, 259, 451,
    // 481, and 703.
    //
    // Find the sum of the first twenty-five composite values of n for which
    // GCD(n, 10) = 1 and n − 1 is divisible by A(n).
    let mut primes: HashSet<u32> = HashSet::new();
    crible235(1000000, |p| {
        primes.insert(p);
    });

    let limit = 25;
    let mut result = 0;
    let mut count = 0;
    for n in (1..).step_by(2) {
        if n % 5 != 0 && !primes.contains(&n) {
            let k = n.repunit_a(10);
            if n % k == 1 {
                count += 1;
                result += n;
                if count >= limit {
                    break;
                }
            }
        }
    }

    result.to_string()
}
