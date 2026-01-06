use num_traits::PrimInt;
use std::collections::{HashMap, VecDeque};
use std::ops::{AddAssign, Div, DivAssign, Mul, MulAssign};

pub trait Digits: Sized + AddAssign + Div + DivAssign + Mul + MulAssign {
    fn loop_digits<F: FnMut(Self)>(self, base: Self, op: F);

    fn extract_digits(self, base: Self) -> VecDeque<Self>;

    fn count_digits(self, base: Self) -> usize;

    fn sum_digits(self, base: Self) -> Self;

    fn palindrome(self, base: Self) -> bool;

    fn ones(self) -> Vec<usize>;

    fn pandigital(self, base: Self) -> bool;

    fn reverse(self, base: Self) -> Self;

    fn count_digit(self, d: Self, base: Self) -> usize;

    fn is_permutation(a: Self, b: Self, base: Self) -> bool;

    fn concat_numbers(a: Self, b: Self, base: Self) -> Self;
}

macro_rules! impl_digits {
    ($($t:ty),*) => {
        $(
            impl Digits for $t {
                fn loop_digits<F: FnMut(Self)>(self, base: Self, mut op: F) {
                    let mut n = self;
                    while n != 0 {
                        op(n % base);
                        n /= base;
                    }
                }

                fn extract_digits(self, base: Self) -> VecDeque<Self> {
                    let mut v: VecDeque<Self> = VecDeque::new();
                    self.loop_digits(base, |digit| v.push_front(digit));
                    v
                }

                fn count_digits(self, base: Self) -> usize {
                    let mut result = 0;
                    self.loop_digits(base, |_| {
                        result += 1;
                    });
                    result
                }

                fn sum_digits(self, base: Self) -> Self {
                    let mut result = 0;
                    self.loop_digits(base, |digit| {
                        result += digit;
                    });
                    result
                }

                fn palindrome(self, base: Self) -> bool
                {
                    let digits = self.extract_digits(base);
                    digits.iter().eq(digits.iter().rev())
                }

                fn ones(self) -> Vec<usize> {
                    let mut n = self;
                    let mut res = Vec::<usize>::new();
                    for i in 0.. {
                        if n % 2 != 0 {
                            res.push(i);
                        }
                        n = n >> 1;
                        if n == 0 {
                            break;
                        }
                    }
                    res
                }

                fn pandigital(self, base: Self) -> bool {
                    let mut digits: HashMap<Self, usize> = HashMap::new();
                    self.loop_digits(base, |digit| {
                        digits.entry(digit).or_insert(0).add_assign(1);
                    });

                    if digits.contains_key(&0) {
                        return false;
                    }

                    digits.values().all(|digit| *digit < 2)
                }

                fn reverse(self, base: Self) -> Self {
                    let mut result = 0;
                    self.loop_digits(base, |d| result = base * result + d);
                    result
                }

                fn count_digit(self, d: Self, base: Self) -> usize {
                    let mut count = 0;
                    self.loop_digits(base, |digit| if digit == d { count += 1; });
                    count
                }

                fn is_permutation(a: Self, b: Self, base: Self) -> bool {
                    let i_base = base as usize;
                    let mut digits_a: Vec<usize> = vec![1; i_base];
                    a.loop_digits(base, |digit| {
                        digits_a[digit as usize] += 1;
                    });
                    let mut digits_b: Vec<usize> = vec![1; i_base];
                    b.loop_digits(base, |digit| {
                        digits_b[digit as usize] += 1;
                    });
                    digits_a == digits_b
                }

                fn concat_numbers(a: Self, b: Self, base: Self) -> Self {
                    let mut multiplier = 1;
                    let mut temp = b;

                    while temp > 0 {
                        multiplier *= base;
                        temp /= base;
                    }

                    a * multiplier + b
                }
                        }
        )*
    }
}

impl_digits!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

pub(crate) fn conversion<'a, N, V>(list: V, base: N) -> N
where
    N: PrimInt + DivAssign + Copy + 'a,
    V: IntoIterator<Item = &'a N>,
{
    let mut result = N::zero();
    for &d in list {
        result = base * result + d;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_digits() {
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

        let digits = 1234567890.extract_digits(10);
        assert_eq!(digits, expected);
    }

    #[test]
    fn test_conversion() {
        let chiffres = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let conversion = conversion(&chiffres, 10);
        assert_eq!(conversion, 1234567890);
    }

    #[test]
    fn test_palindrome() {
        assert_eq!(1234567890.palindrome(10), false);
        assert_eq!(1234567890987654321u64.palindrome(10), true);
    }

    #[test]
    fn test_reverse() {
        assert_eq!(1234567890.reverse(10), 987654321);
        assert_eq!(1234567890987654321u64.reverse(10), 1234567890987654321);
    }

    #[test]
    fn test_permutation() {
        assert_eq!(u64::is_permutation(1234567890, 1237890456, 10), true);
        assert_eq!(u64::is_permutation(1234567890, 1237899456, 10), false);
        assert_eq!(
            u64::is_permutation(1234567890987654321, 1122334455667788990, 10),
            true
        );
    }

    #[test]
    fn test_concat_numbers() {
        assert_eq!(
            u64::concat_numbers(1234567890, 1237890456, 10),
            12345678901237890456u64
        );
        assert_eq!(
            u64::concat_numbers(1234567890, 1237899456, 10),
            12345678901237899456u64
        );
    }

    #[test]
    fn test_count_digits() {
        assert_eq!(1234567890.count_digits(10), 10);
        assert_eq!(12345678901237890456u128.count_digits(10), 20);
    }

    #[test]
    fn test_sum_digits() {
        assert_eq!(1234567890.sum_digits(10), 45);
        assert_eq!(12345678901237890456u64.sum_digits(10), 90);
    }

    #[test]
    fn test_ones() {
        assert_eq!(1234567890.count_ones(), 12);
        assert_eq!(
            1234567890.ones(),
            vec![1, 4, 6, 7, 9, 17, 18, 20, 23, 24, 27, 30]
        );
        assert_eq!(12345678901237890456u64.count_ones(), 33);
        assert_eq!(
            12345678901237890456u64.ones(),
            vec![
                3, 4, 7, 8, 10, 11, 12, 13, 15, 16, 20, 22, 24, 25, 27, 29, 30, 31, 34, 35, 39, 40,
                43, 45, 47, 50, 52, 54, 56, 57, 59, 61, 63
            ]
        );
    }
}
