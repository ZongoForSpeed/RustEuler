use num_traits::{ConstOne, ConstZero, PrimInt};
use std::ops::{MulAssign, SubAssign};

pub(crate) fn factorial<N>(_n: N) -> N
where
    N: PrimInt + MulAssign + SubAssign + ConstOne + ConstZero
{
    let zero = N::ZERO;
    let one = N::ONE;

    //     (1..n + 1).fold(1, |a, b| a * b)
    let mut result = N::ONE;
    let mut n = _n;
    while n > zero {
        result *= n;
        n -= one;
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(10), 3628800);
        assert_eq!(factorial::<u128>(30), 265252859812191058636308480000000);
    }
}
