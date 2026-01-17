use crate::maths::primes::crible23;
use crate::register_problem;
use std::collections::HashSet;

register_problem!(87, "Prime power triples", problem087);

pub fn problem087() -> String {
    // The smallest number expressible as the sum of a prime square, prime cube, and prime fourth
    // power is 28. In fact, there are exactly four numbers below fifty that can be expressed in
    // such a way:
    //
    //  28 = 2² + 2^3 + 2^4
    //  33 = 3² + 2^3 + 2^4
    //  49 = 5² + 2^3 + 2^4
    //  47 = 2² + 3^3 + 2^4
    //
    // How many numbers below fifty million can be expressed as the sum of a prime square, prime
    // cube, and prime fourth power?
    let limit:u32 = 50000000;
    let mut primes:Vec<u32> = Vec::new();
    crible23((limit.isqrt() + 6) as usize, |p| primes.push(p));

    let mut result = HashSet::new();
    let len = primes.len();
    for i in 0..len {
        let p = primes[i];
        let pp = p * p * p * p;
        if pp > limit {
            break;
        }
        for j in 0..len {
            let q = primes[j];
            let qq = q * q * q + pp;
            if qq > limit {
                break;
            }
            for k in 0..len {
                let r = primes[k];
                let n = r * r + qq;
                if n > limit {
                    break;
                }
                result.insert(n);
            }
        }
    }

    result.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem087() {
        let result = problem087();
        assert_eq!(result, "1097343");
    }
}
