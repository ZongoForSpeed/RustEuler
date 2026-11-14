use crate::maths::arithmetic::Arithmetic;
use crate::maths::integer_root::isqrt;
use crate::maths::primes::crible235;
use crate::register_problem;
use std::collections::BTreeMap;

register_problem!(302, "Strong Achilles Numbers", problem302);

fn ajout_facteur(n: u64, resultat: &mut BTreeMap<u64, u64>, premiers: &[u64]) {
    n.factorization(premiers, |p, exposant| {
        *resultat.entry(p).or_insert(0) += exposant as u64;
    });
}

fn algorithme(
    n: u64,
    lastp: u64,
    pgcd: u64,
    decomposition: &BTreeMap<u64, u64>,
    limite: u64,
    premiers: &[u64],
    est_premier: &[bool],
) -> u64 {
    let mut resultat = 0;
    if pgcd == 1 {
        let mut exposant: Option<u64> = None;
        for &e in decomposition.values() {
            if e == 1 {
                exposant = None;
                break;
            }
            exposant = Some(match exposant {
                Some(x) => u64::gcd(x, e),
                None => e,
            });
        }

        if exposant == Some(1) {
            resultat += 1;
        }
    }

    let mut maxp = isqrt(limite / n);
    if maxp >= lastp {
        maxp = lastp - 1;
    }

    let mut minp = 2;
    for (&p, &e) in decomposition.iter().rev() {
        if e < 2 {
            minp = p;
            break;
        }
    }

    for p in (minp..=maxp).rev() {
        if est_premier[p as usize] {
            let mut m = n * p;
            let mut d2 = decomposition.clone();
            ajout_facteur(p - 1, &mut d2, premiers);
            let mut exposant = 2;
            while m <= limite / p {
                m *= p;
                *d2.entry(p).or_insert(0) += 1;
                resultat += algorithme(
                    m,
                    p,
                    u64::gcd(pgcd, exposant),
                    &d2,
                    limite,
                    premiers,
                    est_premier,
                );
                exposant += 1;
            }
        }
    }

    resultat
}

/// A positive integer n is powerful if p² is a divisor of n for every prime factor p in n.
///
/// A positive integer n is a perfect power if n can be expressed as a power of another positive integer.
///
/// A positive integer n is an Achilles number if n is powerful but not a perfect power. For example, 864 and 1800
/// are Achilles numbers:
/// 864 = 2**5·3**3 and 1800 = 2**3·3²·5².
///
/// We shall call a positive integer S a Strong Achilles number if both S and φ(S) are Achilles numbers.1
/// For example, 864 is a Strong Achilles number: φ(864) = 288 = 2**5·3**2.
/// However, 1800 isn't a Strong Achilles number because: φ(1800) = 480 =
/// 2**5·3**1·5**1.
///
/// There are 7 Strong Achilles numbers below 10**4 and 656 below 10**8.
///
/// How many Strong Achilles numbers are there below 10**18?
///
/// 1 φ denotes Euler's totient function.
pub fn problem302() -> String {
    let limite_crible: u64 = 1_000_000 + 1;
    let mut premiers = Vec::new();
    crible235::<u64, _>(limite_crible as usize, |p| premiers.push(p));
    premiers.retain(|&p| p < limite_crible);

    let mut est_premier = vec![false; limite_crible as usize + 1];
    for &p in &premiers {
        est_premier[p as usize] = true;
    }

    let limite: u64 = 10u64.pow(18);

    let resultat = algorithme(
        1,
        limite_crible,
        0,
        &BTreeMap::new(),
        limite,
        &premiers,
        &est_premier,
    );

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem302() {
        assert_eq!(problem302(), "1170060");
    }
}
