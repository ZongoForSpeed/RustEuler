use crate::maths::arithmetic::Arithmetic;
use crate::maths::primes::crible235;
use crate::register_problem;

register_problem!(263, "An engineers' dream come true", problem263);

fn is_practical(n: u64, primes: &[u64]) -> bool {
    // https://fr.wikipedia.org/wiki/Nombre_pratique
    // n > 1 est pratique si et seulement si sa décomposition en facteurs
    // premiers s'écrit n=p_0^a_0.p_1^a_1...p_k^a_k
    // avec k ≥ 0, p0 < p1 < … < pk premiers, α0, … , αk > 0 et
    // Quelquesoit i dans {0, ... ,k} p_i <= 1+sigma(p_0^a_0.p_1^a_1...p_i-1^a_i-1)
    if n % 2 != 0 && n != 1 {
        return false;
    }
    let mut sigma = 1;
    let mut is_p = true;
    n.factorization(primes, |p, e| {
        if !is_p {
            return;
        }
        if p > sigma + 1 {
            is_p = false;
            return;
        }
        let mut p_pow = p;
        let mut sum_p = 1 + p;
        for _ in 1..e {
            p_pow *= p;
            sum_p += p_pow;
        }
        sigma *= sum_p;
    });

    is_p
}

/// Consider the number 6. The divisors of 6 are: 1,2,3 and 6.
/// Every number from 1 up to and including 6 can be written as a sum of distinct divisors of 6:
///      1=1, 2=2, 3=1+2, 4=1+3, 5=2+3, 6=6.
/// A number n is called a practical number if every number from 1 up to and including n can be expressed as a sum of
/// distinct divisors of n.
///
/// A pair of consecutive prime numbers with a difference of six is called a sexy pair (since "sex" is the Latin word
/// for "six"). The first sexy pair is (23, 29).
///
/// We may occasionally find a triple-pair, which means three consecutive sexy prime pairs, such that the second
/// member of each pair is the first member of the next pair.
///
/// We shall call a number n such that :
///      - (n-9, n-3), (n-3,n+3), (n+3, n+9) form a triple-pair, and
///      - the numbers n-8, n-4, n, n+4 and n+8 are all practical,
/// an engineers’ paradise.
///
/// Find the sum of the first four engineers’ paradises.
pub fn problem263() -> String {
    let mut primes = Vec::with_capacity(70_000_000);
    crible235(1_200_000_000, |p| primes.push(p));

    let mut result = 0;
    let mut count = 0;

    for x in primes.windows(4) {
        let p1 = x[0];
        let p2 = x[1];
        let p3 = x[2];
        let p4 = x[3];

        if p2 == p1 + 6 && p3 == p2 + 6 && p4 == p3 + 6 {
            let n = p1 + 9;
            if is_practical(n, &primes)
                && is_practical(n - 4, &primes)
                && is_practical(n + 4, &primes)
                && is_practical(n - 8, &primes)
                && is_practical(n + 8, &primes)
            {
                result += n;
                count += 1;
                if count == 4 {
                    break;
                }
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem263() {
        assert_eq!(problem263(), "2039506520");
    }
}
