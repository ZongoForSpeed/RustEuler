use crate::maths::arithmetic::Arithmetic;
use crate::maths::primes::crible235;
use crate::register_problem;
use bit_set::BitSet;
use std::collections::HashMap;

register_problem!(182, "RSA encryption", problem182);

pub fn problem182() -> String {
    // The RSA encryption is based on the following procedure:
    //
    // Generate two distinct primes p and q.
    // Compute n=pq and φ=(p-1)(q-1).
    // Find an integer e, 1<e<φ, such that gcd(e,φ)=1.
    //
    // A message in this system is a number in the interval [0,n-1].
    // A text to be encrypted is then somehow converted to messages (numbers in the interval [0,n-1]).
    // To encrypt the text, for each message, m, c=m^e mod n is calculated.
    //
    // To decrypt the text, the following procedure is needed: calculate d such that ed=1 mod φ, then for each encrypted
    // message, c, calculate m=c^d mod n.
    //
    // There exist values of e and m such that m^e mod n=m.
    // We call messages m for which m^e mod n=m unconcealed messages.
    //
    // An issue when choosing e is that there should not be too many unconcealed messages.
    // For instance, let p=19 and q=37.
    // Then n=19*37=703 and φ=18*36=648.
    // If we choose e=181, then, although gcd(181,648)=1 it turns out that all possible messages
    // m (0≤m≤n-1) are unconcealed when calculating m^e mod n.
    // For any valid choice of e there exist some unconcealed messages.
    // It's important that the number of unconcealed messages is at a minimum.
    //
    // Choose p=1009 and q=3643.
    // Find the sum of all values of e, 1<e<φ(1009,3643) and gcd(e,φ)=1, so that the number of unconcealed messages for
    // this value of e is at a minimum.
    let p = 1009;
    let q = 3643;
    let n = p * q;

    let mut primes = Vec::new();
    crible235(n, |p| primes.push(p));

    let phi_n = (p - 1) * (q - 1);

    let mut decomposition = HashMap::new();
    phi_n.factorization(&primes, |p, e| {
        decomposition.insert(p, e);
    });

    let mut crible: BitSet<u32> = BitSet::from_iter(1..phi_n);
    crible.remove(0);
    crible.remove(1);

    for (p, _) in decomposition {
        for m in (p..phi_n).step_by(p) {
            crible.remove(m);
        }
    }

    let exponents: Vec<usize> = crible.iter().collect();

    let mut result = 0;
    let mut minimum_collision = n;
    for e in exponents {
        let collision = (usize::gcd(e - 1, p - 1) + 1) * (usize::gcd(e - 1, q - 1) + 1);
        if collision < minimum_collision {
            result = e;
            minimum_collision = collision;
        } else if collision == minimum_collision {
            result += e;
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem182() {
        let result = problem182();
        assert_eq!(result, "399788195976");
    }
}
