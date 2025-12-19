use num_traits::PrimInt;
use num_traits::Zero;
use std::ops::{Div, DivAssign, Mul, MulAssign, Rem};

pub(crate) fn pgcd<N>(_a: N, _b: N) -> N
where
    N: Zero + Rem + Eq + DivAssign + Copy + From<<N as Rem>::Output>,
{
    if _a.is_zero() {
        return _b;
    }
    if _b.is_zero() {
        return _a;
    }

    let mut a = _a;
    let mut b = _b;
    loop {
        let pgcd: N = (a % b).into();
        if pgcd.is_zero() {
            return b;
        }
        a = b;
        b = pgcd;
    }
}

pub(crate) fn ppcm<N>(a: N, b: N) -> N
where
    N: Zero
        + Mul
        + Div
        + Rem
        + Eq
        + DivAssign
        + Copy
        + From<<N as Rem>::Output>
        + From<<N as Div>::Output>
        + From<<N as Mul>::Output>,
{
    let ab: N = (a * b).into();
    (ab / pgcd(a, b)).into()
}

pub(crate) fn number_of_divisors<'a, N, V>(mut _n: N, primes: V) -> u32
where
    N: PrimInt + DivAssign + Copy + 'a,
    V: IntoIterator<Item = &'a N>,
{
    let zero = N::zero();
    let mut n = _n;
    let mut d: u32 = 1;
    for &p in primes {
        if p * p > n {
            break;
        }
        if n % p == zero {
            let mut compteur: u32 = 0;
            while (n % p) == zero {
                n /= p;
                compteur += 1;
            }
            d *= compteur + 1;
        }
    }

    if n > N::one() {
        d *= 2;
    }

    d
}

pub(crate) fn factorization<'a, N, V, F>(_n: N, primes: V, mut output: F)
where
    N: PrimInt + DivAssign + Copy + 'a,
    V: IntoIterator<Item = &'a N>,
    F: FnMut(N, usize),
{
    let mut n = _n;
    for &p in primes {
        if p * p > n {
            break;
        }
        if (n % p).is_zero() {
            let mut count: usize = 0;
            while (n % p).is_zero() {
                n /= p;
                count += 1;
            }
            output(p, count);
        }
    }

    if n > N::one() {
        output(n, 1);
    }
}

pub(crate) fn sum_of_divisors<'a, N, V>(mut _n: N, primes: V) -> N
where
    N: PrimInt + MulAssign + DivAssign + Copy + 'a,
    V: IntoIterator<Item = &'a N>,
{
    let one = N::one();
    let mut s = N::one();
    let mut n = _n;
    for &p in primes {
        if p * p > n {
            break;
        }
        if (n % p).is_zero() {
            let mut count: u32 = 0;
            while (n % p).is_zero() {
                n /= p;
                count += 1;
            }
            s *= (p.pow(count + 1) - one) / (p - one);
        }
    }

    if n > N::one() {
        s *= n + one;
    }
    s
}

pub(crate) fn phi<'a, N, V>(mut _n: N, primes: V) -> N
where
    N: PrimInt + MulAssign + DivAssign + Copy + 'a,
    V: IntoIterator<Item = &'a N>,
{
    let mut result = _n;
    let mut n = _n;
    for &p in primes {
        if p * p > n {
            break;
        }
        if (n % p).is_zero() {
            result = result - result / p;
            while (n % p).is_zero() {
                n /= p;
            }
        }
    }

    let one = N::one();
    if n > one {
        result = result - result / n;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_pgcd() {
        assert_eq!(pgcd(456753, 97643), 1);
        assert_eq!(pgcd(456755, 158665), 65);
    }

    #[test]
    fn test_ppcm() {
        assert_eq!(ppcm(456753u64, 97643u64), 44598733179u64);
        assert_eq!(ppcm(456755u64, 158665u64), 1114938955u64);
    }

    #[test]
    fn test_number_of_divisors() {
        let primes: Vec<u64> = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];
        assert_eq!(number_of_divisors(456753, &primes), 8);
        assert_eq!(number_of_divisors(3246999210, &primes), 640);
    }
    #[test]
    fn test_sum_of_divisors() {
        let primes: Vec<u64> = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];
        assert_eq!(sum_of_divisors(456753, &primes), 664416);
        assert_eq!(sum_of_divisors(496, &primes), 992);
        assert_eq!(sum_of_divisors(3246999210u64, &primes), 11708928000);
    }
    #[test]
    fn test_phi() {
        let primes: Vec<u64> = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
            31, 37, 41, 43, 47, 53, 59, 61, 67,
            71, 73, 79, 83, 89, 97
        ];
        assert_eq!(phi(3246999210, &primes), 640120320);
        assert_eq!(phi(496, &primes), 240);
    }
    #[test]
    fn test_factorization() {
        let primes: Vec<u64> = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];
        let mut d: HashMap<u64, usize> = HashMap::new();
        factorization(3246999210u64, &primes, |p, c| {
            d.insert(p, c);
        });
        println!("{:?}", d);
        let expected = HashMap::from([(29, 1), (7, 3), (13, 1), (3, 4), (5, 1), (31, 1), (2, 1)]);
        assert_eq!(d, expected);
    }
}
