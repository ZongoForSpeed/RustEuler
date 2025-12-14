use num_traits::PrimInt;
use std::collections::VecDeque;
use std::ops::{DivAssign, Mul};

pub(crate) fn boucle_chiffre<T, F>(mut n: T, base: T, mut op: F)
where
    T: PrimInt + DivAssign,
    F: FnMut(T),
{
    let zero = T::from(0u8).unwrap();
    while n != zero {
        op(n % base);
        n /= base;
    }
}

pub(crate) fn extraire_chiffres<T>(n: T, base: T) -> VecDeque<T>
where
    T: PrimInt + std::ops::DivAssign,
{
    let mut v: VecDeque<T> = VecDeque::new();
    boucle_chiffre(n, base, |digit| {
        v.push_front(digit);
    });
    v
}

pub(crate) fn palindrome<T>(n: T, base: T) -> bool
where
    T: PrimInt + DivAssign,
{
    let chiffres = extraire_chiffres(n, base);
    chiffres.iter().eq(chiffres.iter().rev())
}

fn addmul<'a, N>(a: N, b: N, base: N) -> N
where
    N: PrimInt + Mul + DivAssign + Copy + 'a,
{
    base * a + b
}

pub(crate) fn conversion<'a, N, V>(list: V, base: N) -> N
where
    N: PrimInt + DivAssign + Copy + 'a,
    V: IntoIterator<Item = &'a N>,
{
    list.into_iter()
        .map(N::clone)
        .reduce(|a, b| addmul(a, b, base))
        .unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extraire_chiffres() {
        let chiffres = extraire_chiffres(1234567890, 10);
        println!("chiffres = {:?}", chiffres);
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        assert_eq!(chiffres, expected);
    }

    #[test]
    fn test_conversion() {
        let chiffres = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let conversion = conversion(&chiffres, 10);
        assert_eq!(conversion, 1234567890);
    }

    #[test]
    fn test_palindrome() {
        assert_eq!(palindrome(1234567890, 10), false);
        assert_eq!(palindrome(1234567890987654321u64, 10), true);
    }

}
