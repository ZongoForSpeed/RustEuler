use std::ops::{AddAssign, Div, DivAssign, Mul, MulAssign};

pub(crate) trait Factorial: Sized + AddAssign + Div + DivAssign + Mul + MulAssign {
    fn factorial(n: Self) -> Self;

    fn binomial(n: Self, p: Self) -> Self;
}

macro_rules! impl_factorial {
    ($($t:ty),*) => {
        $(
            impl Factorial for $t {
                fn factorial(mut n: Self) -> Self {
                    //     (1..n + 1).fold(1, |a, b| a * b)
                    let mut result = 1;
                    while n > 0 {
                        result *= n;
                        n -= 1;
                    }
                    result
                }

                fn binomial(n: Self, mut p: Self) -> Self {
                    if (p < 1 || n < p) {
                        return 1;
                    }

                    if (p > n / 2) {
                        p = n - p;
                    }

                    let mut numer = 1;
                    let mut denom = 1;

                    for k in 0..p {
                        numer *= (n - k);
                        denom *= (k + 1);
                    }

                    numer / denom

                }
            }
        )*
    }
}

impl_factorial!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(u32::factorial(10), 3628800);
        assert_eq!(u128::factorial(30), 265252859812191058636308480000000);
    }

    #[test]
    fn test_binomial() {
        assert_eq!(35, u32::binomial(7, 3));
        assert_eq!(70, u32::binomial(8, 4));
        assert_eq!(1, u32::binomial(40, 0));
        assert_eq!(137846528820, u128::binomial(40, 20));
    }

}
