use crate::maths::primes::crible2;
use crate::register_problem;

register_problem!(
    278,
    "Linear Combinations of Semiprimes",
    problem278
);

/// Given the values of integers 1 < a1 < a2 <... < an, consider the linear combination q1.a1 + q2.a2 + ...
/// + qn.an = b, using only integer values qk ≥ 0.
///
/// Note that for a given set of ak, it may be that not all values of b are possible.
/// For instance, if a1 = 5 and a2 = 7, there are no q1 ≥ 0 and q2 ≥ 0 such that b could be 1, 2, 3, 4, 6, 8, 9, 11,
/// 13, 16, 18 or 23.
///
/// In fact, 23 is the largest impossible value of b for a1 = 5 and a2 = 7. We therefore call f(5, 7) = 23.
///
/// Similarly, it can be shown that f(6, 10, 15)=29 and f(14, 22, 77) = 195.
///
/// Find ∑ f(p*q,p*r,q*r), where p, q and r are prime numbers and p < q < r < 5000.
pub fn problem278() -> String {
    let mut premiers = Vec::new();
    crible2::<u64, _>(5000, |p| premiers.push(p));

    let n = premiers.len();
    let mut resultat = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let (p, q, r) = (premiers[i], premiers[j], premiers[k]);
                // https://fr.wikipedia.org/wiki/Probl%C3%A8me_des_pi%C3%A8ces_de_monnaie
                resultat += p * q * r * 2 - p * q - q * r - p * r;
            }
        }
    }

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem278() {
        assert_eq!(problem278(), "1228215747273908452");
    }
}
