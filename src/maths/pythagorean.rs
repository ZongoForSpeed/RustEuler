use std::iter::successors;
use crate::maths::arithmetique::pgcd;
use num_traits::PrimInt;
use std::ops::{AddAssign, DivAssign, Rem};

pub(crate) fn pythagorean<N>() -> impl Iterator<Item = (N, N, N)>
where
    N: PrimInt + AddAssign + DivAssign,
{
    successors(Some((N::one() + N::one(), N::one())), next_pythagorean)
    .map(|(p, q)| build_triplet(p, q))
}

fn next_pythagorean<N>(t: &(N, N)) -> Option<(N, N)>
where
    N: PrimInt + AddAssign + DivAssign,
{
    let mut p = t.0;
    let mut q = t.1;
    let two = N::one() + N::one();
    loop {
        q += two;
        if q > p {
            p += N::one();
            q = p % two + N::one();
        }
        if pgcd(p, q) == N::one() {
            break;
        }
    }
    Some((p, q))
}

fn next_pythagorean_limit<N>(t: &(N, N, N)) -> Option<(N, N, N)>
where
    N: PrimInt + AddAssign + DivAssign,
{
    let mut p = t.0;
    let mut q = t.1;
    let limit = t.2;
    let two = N::one() + N::one();
    loop {
        q += two;
        if q > p || p * p + q * q >= limit {
            p += N::one();
            q = p % two + N::one();
            if p * p >= limit {
                return None;
            }
        }
        if pgcd(p, q) == N::one() {
            break;
        }
    }
    Some((p, q, limit))
}

fn build_triplet<N>(p: N, q: N) -> (N, N, N)
where
    N: PrimInt + AddAssign + Rem + DivAssign,
{
    let two = N::one() + N::one();
    let a = p * p - q * q;
    let b = two * p * q;
    let c = p * p + q * q;
    if a < b { (a, b, c) } else { (b, a, c) }
}

pub(crate) fn pythagorean_limit<N>(limite: N) -> impl Iterator<Item = (N, N, N)>
where
    N: PrimInt + AddAssign + DivAssign + 'static,
{
    successors(Some((N::one() + N::one(), N::one(), limite)), next_pythagorean_limit)
    .map(|(p, q, _)| build_triplet(p, q))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pythagorean() {
        let expected = vec![
            (3, 4, 5),
            (5, 12, 13),
            (8, 15, 17),
            (7, 24, 25),
            (20, 21, 29),
            (9, 40, 41),
            (12, 35, 37),
            (11, 60, 61),
            (28, 45, 53),
            (33, 56, 65),
        ];
        let result: Vec<(u64, u64, u64)> = pythagorean().take(10).collect();
        println!("{:?}", result);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_fibonacci_limite() {
        let expected = vec![
            (3, 4, 5),
            (5, 12, 13),
            (8, 15, 17),
            (7, 24, 25),
            (20, 21, 29),
            (9, 40, 41),
            (12, 35, 37),
            (28, 45, 53),
        ];
        let result: Vec<(u64, u64, u64)> = pythagorean_limit(50).collect();
        println!("{:?}", result);

        assert_eq!(result, expected);
    }
}
