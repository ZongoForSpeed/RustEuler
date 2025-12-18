use num_traits::PrimInt;
use std::collections::VecDeque;
use std::ops::{AddAssign, Div, DivAssign, Mul, MulAssign};

pub(crate) fn loop_digits<Nombre, F>(mut n: Nombre, base: Nombre, mut op: F)
where
    Nombre: PrimInt + DivAssign,
    F: FnMut(Nombre),
{
    let zero = Nombre::zero();
    while n != zero {
        op(n % base);
        n /= base;
    }
}

pub(crate) fn extract_digits<T>(n: T, base: T) -> VecDeque<T>
where
    T: PrimInt + DivAssign,
{
    let mut v: VecDeque<T> = VecDeque::new();
    loop_digits(n, base, |digit| v.push_front(digit));
    v
}

pub(crate) fn count_digits<T: PrimInt + DivAssign + AddAssign>(n: T, base: T) -> T {
    let mut result = T::zero();
    let one = T::one();
    loop_digits(n, base, |_| {
        result += one;
    });
    result
}

pub(crate) fn sum_digits<T: PrimInt + DivAssign + AddAssign>(n: T, base: T) -> T {
    let mut result = T::zero();
    loop_digits(n, base, |digit| {
        result += digit;
    });
    result
}

pub(crate) fn palindrome<T>(n: T, base: T) -> bool
where
    T: PrimInt + DivAssign,
{
    let digits = extract_digits(n, base);
    digits.iter().eq(digits.iter().rev())
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

pub(crate) fn is_permutation<N: PrimInt + AddAssign + DivAssign + Copy>(
    a: N,
    b: N,
    base: N,
) -> bool {
    let i_base = base.to_usize().unwrap();
    let mut digits_a: Vec<N> = vec![N::zero(); i_base];
    loop_digits(a, base, |digit| {
        digits_a[digit.to_usize().unwrap()] += N::one();
    });
    let mut digits_b: Vec<N> = vec![N::zero(); i_base];
    loop_digits(b, base, |digit| {
        digits_b[digit.to_usize().unwrap()] += N::one();
    });
    //chiffres_a.iter().eq(chiffres_b.iter())
    digits_a == digits_b
}

pub(crate) fn concat_numbers<T: PrimInt + Div + DivAssign + MulAssign>(a: T, b: T, base: T) -> T {
    let mut multiplier = T::one();
    let mut temp = b;

    while temp > T::zero() {
        multiplier *= base;
        temp /= base;
    }

    a * multiplier + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_digits() {
        let chiffres = extract_digits(1234567890, 10);
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

    #[test]
    fn test_permutation() {
        assert_eq!(is_permutation(1234567890, 1237890456, 10), true);
        assert_eq!(is_permutation(1234567890, 1237899456, 10), false);
        assert_eq!(
            is_permutation(1234567890987654321u64, 1122334455667788990u64, 10),
            true
        );
    }

    #[test]
    fn test_concat_numbers() {
        assert_eq!(
            concat_numbers(1234567890, 1237890456, 10),
            12345678901237890456u64
        );
        assert_eq!(
            concat_numbers(1234567890, 1237899456, 10),
            12345678901237899456u64
        );
    }

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(1234567890, 10), 10);
        assert_eq!(count_digits(12345678901237890456u128, 10), 20);
    }
    #[test]
    fn test_sum_digits() {
        assert_eq!(sum_digits(1234567890, 10), 45);
        assert_eq!(sum_digits(12345678901237890456u64, 10), 90);
    }
}
