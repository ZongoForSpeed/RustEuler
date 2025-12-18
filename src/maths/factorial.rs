use num_traits::PrimInt;
use std::ops::{MulAssign, SubAssign};

pub(crate) fn factorial<Nombre>(_n: Nombre) -> Nombre
where
    Nombre: PrimInt + MulAssign + SubAssign
{
    let zero = Nombre::zero();
    let one = Nombre::one();

    //     (1..n + 1).fold(1, |a, b| a * b)
    let mut result = Nombre::one();
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
