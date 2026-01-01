use crate::maths::arithmetic::Arithmetic;
use num_traits::{ConstOne, PrimInt};
use std::iter::successors;
use std::ops::{AddAssign, DivAssign, Rem};

pub(crate) fn pythagorean<N>() -> impl Iterator<Item = (N, N, N)>
where
    N: PrimInt + AddAssign + DivAssign + Arithmetic + ConstOne,
{
    successors(Some((N::ONE + N::ONE, N::ONE)), next_pythagorean).map(|(p, q)| build_triplet(p, q))
}

fn next_pythagorean<N>(t: &(N, N)) -> Option<(N, N)>
where
    N: PrimInt + AddAssign + DivAssign + Arithmetic + ConstOne,
{
    let mut p = t.0;
    let mut q = t.1;
    let two = N::ONE + N::ONE;
    loop {
        q += two;
        if q > p {
            p += N::ONE;
            q = p % two + N::ONE;
        }
        if N::gcd(p, q) == N::ONE {
            break;
        }
    }
    Some((p, q))
}

fn next_pythagorean_limit<N>(t: &(N, N, N)) -> Option<(N, N, N)>
where
    N: PrimInt + AddAssign + DivAssign + Arithmetic + ConstOne,
{
    let mut p = t.0;
    let mut q = t.1;
    let limit = t.2;
    let two = N::ONE + N::ONE;
    loop {
        q += two;
        if q > p || p * p + q * q >= limit {
            p += N::ONE;
            q = p % two + N::ONE;
            if p * p >= limit {
                return None;
            }
        }
        if N::gcd(p, q) == N::ONE {
            break;
        }
    }
    Some((p, q, limit))
}

fn build_triplet<N>(p: N, q: N) -> (N, N, N)
where
    N: PrimInt + AddAssign + Rem + DivAssign + ConstOne,
{
    let two = N::ONE + N::ONE;
    let a = p * p - q * q;
    let b = two * p * q;
    let c = p * p + q * q;
    if a < b { (a, b, c) } else { (b, a, c) }
}

pub(crate) fn pythagorean_limit<N>(limite: N) -> impl Iterator<Item = (N, N, N)>
where
    N: PrimInt + AddAssign + DivAssign + 'static + Arithmetic + ConstOne,
{
    successors(
        Some((N::ONE + N::ONE, N::ONE, limite)),
        next_pythagorean_limit,
    )
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
