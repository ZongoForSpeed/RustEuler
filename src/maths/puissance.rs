use std::ops::{DivAssign, MulAssign, RemAssign};
use num_traits::PrimInt;

pub(crate) fn puissance<Nombre, Exposant>(_base: Nombre, _exposant: Exposant) -> Nombre
where
    Nombre: PrimInt + DivAssign + MulAssign,
    Exposant: PrimInt + DivAssign + MulAssign,
{
    let zero = Exposant::zero();
    let two = Exposant::one() + Exposant::one();
    let mut resultat = Nombre::one();
    let mut base = _base;
    let mut exposant = _exposant;
    while exposant > zero {
        if exposant % two != zero {
            resultat *= base;
        }
        exposant /= two;
        base *= base;
    }
    resultat
}

pub(crate) fn puissance_m<Nombre, Exposant>(_base: Nombre, _exposant: Exposant, _modulo: Nombre) -> Nombre
where
    Nombre: PrimInt + DivAssign + MulAssign + RemAssign,
    Exposant: PrimInt + DivAssign + MulAssign,
{
    let zero = Exposant::zero();
    let two = Exposant::one() + Exposant::one();
    let mut resultat = Nombre::one();
    let mut base = _base;
    let mut exposant = _exposant;
    while exposant > zero {
        if exposant % two != zero {
            resultat *= base;
            resultat %= _modulo;
        }
        exposant /= two;
        base *= base;
        base %= _modulo;
    }
    resultat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puissance() {
        assert_eq!(puissance(2, 10), 1024);
    }

}
