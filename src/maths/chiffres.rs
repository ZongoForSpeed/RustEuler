use num_traits::PrimInt;
use std::collections::VecDeque;

pub(crate) fn boucle_chiffre<T, F>(mut n: T, base: T, mut op: F)
where
    T: PrimInt + std::ops::DivAssign,
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
    T: PrimInt + std::ops::DivAssign,
{
    let chiffres = extraire_chiffres(n, base);
    chiffres.iter().eq(chiffres.iter().rev())
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
    fn test_palindrome() {
        assert_eq!(palindrome(1234567890, 10), false);
        assert_eq!(palindrome(1234567890987654321u64, 10), true);
    }
}
