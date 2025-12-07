use crate::maths::puissance::puissance;
use num_traits::PrimInt;
use num_traits::Zero;
use std::ops::{Div, DivAssign, Mul, MulAssign, Rem};

pub(crate) fn pgcd<Nombre>(_a: Nombre, _b: Nombre) -> Nombre
where
    Nombre: Zero + Rem + Eq + DivAssign + Copy + From<<Nombre as Rem>::Output>,
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
        let pgcd: Nombre = (a % b).into();
        if pgcd.is_zero() {
            return b;
        }
        a = b;
        b = pgcd;
    }
}

pub(crate) fn ppcm<Nombre>(a: Nombre, b: Nombre) -> Nombre
where
    Nombre: Zero
        + Mul
        + Div
        + Rem
        + Eq
        + DivAssign
        + Copy
        + From<<Nombre as Rem>::Output>
        + From<<Nombre as Div>::Output>
        + From<<Nombre as Mul>::Output>,
{
    let ab: Nombre = (a * b).into();
    (ab / pgcd(a, b)).into()
}

pub(crate) fn nombre_diviseurs<'a, N, V>(mut _n: N, premiers: V) -> u32
where
    N: PrimInt + DivAssign + Copy + 'a,
    V: IntoIterator<Item = &'a N>,
{
    let zero = N::zero();
    let mut n = _n;
    let mut d: u32 = 1;
    for &p in premiers {
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

pub(crate) fn somme_diviseurs<'a, N, V>(mut _n: N, premiers: V) -> N
where
    N: PrimInt + MulAssign + DivAssign + Copy + 'a,
    V: IntoIterator<Item = &'a N>,
{
    let one = N::one();
    let mut s = N::one();
    let mut n = _n;
    for &p in premiers {
        if p * p > n {
            break;
        }
        if (n % p).is_zero() {
            let mut compteur: u32 = 0;
            while (n % p).is_zero() {
                n /= p;
                compteur += 1;
            }
            s *= (puissance(p, compteur + 1) - one) / (p - one);
        }
    }

    if n > N::one() {
        s *= n + one;
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_nombre_diviseurs() {
        let premiers: Vec<u64> = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];
        assert_eq!(nombre_diviseurs(456753, &premiers), 8);
        assert_eq!(nombre_diviseurs(3246999210, &premiers), 640);
    }
    #[test]
    fn test_somme_diviseurs() {
        let premiers: Vec<u64> = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];
        assert_eq!(somme_diviseurs(456753, &premiers), 664416);
        assert_eq!(somme_diviseurs(496, &premiers), 992);
        assert_eq!(somme_diviseurs(3246999210u64, &premiers), 11708928000);
    }
}
