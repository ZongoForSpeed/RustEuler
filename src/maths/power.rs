use num_traits::PrimInt;
use std::ops::{DivAssign, MulAssign, RemAssign};

pub(crate) fn power<Nombre, Exposant>(_base: Nombre, _exponent: Exposant) -> Nombre
where
    Nombre: PrimInt + DivAssign + MulAssign,
    Exposant: PrimInt + DivAssign + MulAssign,
{
    // TODO replace with pow(...)
    // return _base.pow(_exposant as u32);
    let zero = Exposant::zero();
    let two = Exposant::one() + Exposant::one();

    let mut result = Nombre::one();
    let mut base = _base;
    let mut exponent = _exponent;

    while exponent > zero {
        if exponent % two != zero {
            result *= base;
        }
        exponent /= two;
        base *= base;
    }
    result
}

pub(crate) fn power_mod<N, E>(_base: N, _exponent: E, modulo: N) -> N
where
    N: PrimInt + DivAssign + MulAssign + RemAssign,
    E: PrimInt + DivAssign + MulAssign,
{
    let zero = E::zero();
    let two = E::one() + E::one();
    let mut result = N::one();
    let mut base = _base;
    let mut exponent = _exponent;
    while exponent > zero {
        if exponent % two != zero {
            result *= base;
            result %= modulo;
        }
        exponent /= two;
        base *= base;
        base %= modulo;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power() {
        assert_eq!(power(2, 10), 1024);
    }

    #[test]
    fn test_power_mod() {
        assert_eq!(power_mod(2, 10, 25), 24);
    }
}
