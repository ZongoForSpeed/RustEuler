use std::ops::DivAssign;
use num_traits::PrimInt;

pub(crate) fn puissance<Nombre, Exposant>(_base: Nombre, _exposant: Exposant) -> Nombre
where
    Nombre: PrimInt + DivAssign + std::ops::MulAssign,
    Exposant: PrimInt + DivAssign + std::ops::MulAssign,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puissance() {
        assert_eq!(puissance(2, 10), 1024);
    }

}
