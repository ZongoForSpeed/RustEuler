use ndarray::Array2;
use std::ops::{AddAssign, Div, DivAssign, Mul, MulAssign};

pub trait Matrix: Sized + AddAssign + Div + DivAssign + Mul + MulAssign {
    fn power_matrix(a: &Array2<Self>, exponent: usize) -> Array2<Self>;

    fn power_matrix_mod(a: &Array2<Self>, exponent: usize, modulo: Self) -> Array2<Self>;
}

macro_rules! impl_matrix {
    ($($t:ty),*) => {
        $(
            impl Matrix for $t {

                fn power_matrix(a: &Array2<Self>, mut exponent: usize) -> Array2<Self> {
                    let mut base = a.clone();
                    let size = a.shape()[0];
                    let mut result = Array2::<Self>::eye(size);
                    while exponent > 0 {
                        if exponent % 2 == 1 {
                            result = result.dot(&base);
                        }
                        exponent /= 2;
                        base = base.dot(&base);
                    }
                    result
                }

                fn power_matrix_mod(a: &Array2<Self>, mut exponent: usize, modulo: Self) -> Array2<Self> {
                    let mut base = a.clone();
                    let size = a.shape()[0];
                    let mut result = Array2::<Self>::eye(size);
                    while exponent > 0 {
                        if exponent % 2 == 1 {
                            result = result.dot(&base);
                            result %= modulo
                        }
                        exponent /= 2;
                        base = base.dot(&base);
                        base %= modulo;
                    }
                    result
                }
            }
        )*
    }
}

impl_matrix!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_power_matrix_2x2() {
        // Fibonacci matrix: [[1, 1], [1, 0]]^n = [[F_{n+1}, F_n], [F_n, F_{n-1}]]
        let a = array![[1, 1], [1, 0]];
        let result = u32::power_matrix(&a, 5);
        let expected = array![[8, 5], [5, 3]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_power_matrix_3x3() {
        let a = array![[1, 2, 3], [0, 1, 4], [0, 0, 1]];
        // (I + N)^2 = I + 2N + N^2 where N is strictly upper triangular
        let result = u32::power_matrix(&a, 2);
        let expected = array![[1, 4, 14], [0, 1, 8], [0, 0, 1]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_power_matrix_4x4() {
        let a = array![[2, 0, 0, 0], [0, 3, 0, 0], [0, 0, 4, 0], [0, 0, 0, 5]];
        let result = u64::power_matrix(&a, 3);
        let expected = array![[8, 0, 0, 0], [0, 27, 0, 0], [0, 0, 64, 0], [0, 0, 0, 125]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_power_matrix_mod() {
        let a = array![[1, 1], [1, 0]];
        // 10th Fibonacci number is 55. 55 % 7 = 6.
        // [[1, 1], [1, 0]]^10 = [[89, 55], [55, 34]]
        let result = u32::power_matrix_mod(&a, 10, 7);
        let expected = array![[89 % 7, 55 % 7], [55 % 7, 34 % 7]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_power_matrix_f64() {
        let a = array![
            [0.5, 0.0],
            [0.0, 2.0]
        ];
        let result = f64::power_matrix(&a, 3);
        let expected = array![
            [0.125, 0.0],
            [0.0, 8.0]
        ];
        
        for ((r, c), &val) in result.indexed_iter() {
            let exp = expected[[r, c]];
            assert!((val - exp).abs() < 1e-10, "At [{}, {}]: {} != {}", r, c, val, exp);
        }
    }
}
