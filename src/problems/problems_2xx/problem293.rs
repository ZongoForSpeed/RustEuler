use crate::maths::primes::crible2;
use crate::register_problem;
use std::collections::BTreeSet;

register_problem!(
    293,
    "Pseudo-Fortunate Numbers",
    problem293
);

fn est_premier(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }

    true
}

/// Plus petit nombre premier strictement supérieur à x.
fn premier_apres(x: u64) -> u64 {
    let mut m = x + 1;
    while !est_premier(m) {
        m += 1;
    }

    m
}

/// An even positive integer N will be called admissible, if it is a power of 2 or its distinct prime factors are
/// consecutive primes. The first twelve admissible numbers are 2,4,6,8,12,16,18,24,30,32,36,48.
///
/// If N is admissible, the smallest integer M > 1 such that N+M is prime, will be called the pseudo-Fortunate number
/// for N.
///
/// For example, N=630 is admissible since it is even and its distinct prime factors are the consecutive primes 2,3,5
/// and 7. The next prime number after 631 is 641; hence, the pseudo-Fortunate number for 630 is M=11. It can also be
/// seen that the pseudo-Fortunate number for 16 is 3.
///
/// Find the sum of all distinct pseudo-Fortunate numbers for admissible numbers N less than 10**9.
pub fn problem293() -> String {
    let mut premiers = Vec::new();
    crible2::<u64, _>(50, |p| premiers.push(p));

    let limite: u64 = 1_000_000_000;

    let mut fortunate = Vec::new();
    let mut p = premiers[0];
    while p < limite {
        fortunate.push(p);
        p *= premiers[0];
    }

    let mut pseudo_fortunate = BTreeSet::new();
    for &p in &premiers {
        let mut suivant = Vec::new();
        let mut produit = p;
        while produit < limite {
            for &n in &fortunate {
                let f = produit * n;
                if f >= limite {
                    break;
                }
                suivant.push(f);
            }
            produit *= p;
        }

        if suivant.is_empty() {
            break;
        }

        suivant.sort();

        for &n in &suivant {
            let m = premier_apres(n + 1) - n;
            pseudo_fortunate.insert(m);
        }

        fortunate = suivant;
    }

    let resultat: u64 = pseudo_fortunate.iter().sum();

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem293() {
        assert_eq!(problem293(), "2209");
    }
}
