use crate::maths::arithmetique::sum_of_divisors;
use crate::maths::primes::crible23;
use crate::register_problem;
use std::collections::HashSet;

register_problem!(95, "Amicable chains", problem095);

pub fn problem095() -> String {
    // The proper divisors of a number are all the divisors excluding the number itself. For example, the proper
    // divisors of 28 are 1, 2, 4, 7, and 14. As the sum of these divisors is equal to 28, we call it a perfect number.
    //
    // Interestingly the sum of the proper divisors of 220 is 284 and the sum of the proper divisors of 284 is 220,
    // forming a chain of two numbers. For this reason, 220 and 284 are called an amicable pair.
    //
    // Perhaps less well known are longer chains. For example, starting with 12496, we form a chain of five numbers:
    //
    //                  12496 → 14288 → 15472 → 14536 → 14264 (→ 12496 → ...)
    //
    // Since this chain returns to its starting point, it is called an amicable chain.
    //
    // Find the smallest member of the longest amicable chain with no element exceeding one million.
    let limit:usize = 1000000;

    let mut primes:Vec<usize> = Vec::new();
    crible23(limit as usize, |p| primes.push(p));

    let mut sum_divisors = vec![0; limit];
    for n in 2..limit {
        sum_divisors[n] = sum_of_divisors(n, &primes) - n;
    }

    let mut maximum = 0;
    let mut result = 0;
    for n in 2..limit {
        let mut s = n;
        let mut chain = HashSet::new();
        while s < limit {
            if chain.insert(s) {
                s = sum_divisors[s];
            } else {
                break;
            }
        }

        if s == n && chain.len() > maximum {
            maximum = chain.len();
            result = n;
        }
    }

    result.to_string()
}
