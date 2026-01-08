use std::collections::HashMap;
use std::ops::{AddAssign, Div, DivAssign, Mul, MulAssign};

pub(crate) trait Arithmetic: Sized + AddAssign + Div + DivAssign + Mul + MulAssign {
    fn gcd(a: Self, b: Self) -> Self;

    fn lcm(a: Self, b: Self) -> Self;

    fn number_of_divisors<'a, V: IntoIterator<Item = &'a Self>>(self, primes: V) -> Self
    where
        Self: 'a;

    fn sum_of_divisors<'a, V: IntoIterator<Item = &'a Self>>(self, primes: V) -> Self
    where
        Self: 'a;

    fn phi<'a, V: IntoIterator<Item = &'a Self>>(self, primes: V) -> Self
    where
        Self: 'a;

    fn radical<'a, V: IntoIterator<Item = &'a Self>>(self, primes: V) -> Self
    where
        Self: 'a;

    fn mobius<'a, V: IntoIterator<Item = &'a Self>>(self, primes: V) -> i8
    where
        Self: 'a;

    fn factorization<'a, V, F>(self, primes: V, output: F)
    where
        V: IntoIterator<Item = &'a Self>,
        F: FnMut(Self, usize),
        Self: 'a;

    fn divisors<'a, V: IntoIterator<Item = &'a Self>>(self, primes: V) -> Vec<Self>
    where
        Self: 'a;

    fn repunit_a(self, base: Self) -> Self;

    fn count_factor(self, d: Self) -> usize;
}

macro_rules! impl_arithmetic {
    ($($t:ty),*) => {
        $(
            impl Arithmetic for $t {
                fn gcd(_a: Self, _b: Self) -> Self {
                    if _a == 0 {
                        return _b;
                    }
                    if _b == 0 {
                        return _a;
                    }

                    let mut a = _a;
                    let mut b = _b;
                    loop {
                        let pgcd = a % b;
                        if pgcd == 0 {
                            return b;
                        }
                        a = b;
                        b = pgcd;
                    }
                }

                fn lcm(a: Self, b: Self) -> Self {
                    (a * b) / Self::gcd(a, b)
                }

                fn number_of_divisors<'a, V: IntoIterator<Item = &'a Self>>(self, primes: V) -> Self where Self: 'a {
                    let mut n = self;
                    let mut d = 1;
                    for &p in primes {
                        if p * p > n {
                            break;
                        }
                        if n % p == 0 {
                            let mut compteur = 0;
                            while (n % p) == 0 {
                                n /= p;
                                compteur += 1;
                            }
                            d *= compteur + 1;
                        }
                    }

                    if n > 1 {
                        d *= 2;
                    }

                    d
                }

                fn sum_of_divisors<'a, V: IntoIterator<Item = &'a Self>>(self, primes: V) -> Self where Self: 'a {
                    let one = 1;
                    let mut s = 1;
                    let mut n = self;
                    for &p in primes {
                        if p * p > n {
                            break;
                        }
                        if (n % p) == 0 {
                            let mut count: u32 = 0;
                            while (n % p) == 0 {
                                n /= p;
                                count += 1;
                            }
                            s *= (p.pow(count + 1) - one) / (p - one);
                        }
                    }

                    if n > 1 {
                        s *= n + one;
                    }
                    s
                }

                fn phi<'a, V: IntoIterator<Item = &'a Self>>(self, primes: V) -> Self where Self: 'a {
                    let mut result = self;
                    let mut n = self;
                    for &p in primes {
                        if p * p > n {
                            break;
                        }
                        if (n % p) == 0 {
                            result = result - result / p;
                            while (n % p) == 0 {
                                n /= p;
                            }
                        }
                    }

                    if n > 1 {
                        result = result - result / n;
                    }
                    result
                }

                fn radical<'a, V: IntoIterator<Item = &'a Self>>(self, primes: V) -> Self where Self: 'a {
                    let mut result = 1;
                    let mut n = self;
                    for &p in primes {
                        if p * p > n {
                            break;
                        }
                        if (n % p) == 0 {
                            while (n % p) == 0 {
                                n /= p;
                            }
                            result *= p;
                        }
                    }

                    if n > 1 {
                        result *= n;
                    }
                    result
                }

                fn mobius<'a, V: IntoIterator<Item = &'a Self>>(self, primes: V) -> i8 where Self: 'a {
                    let mut factors = 0;
                    let mut n = self;
                    for &p in primes {
                        if p * p > n {
                            break;
                        }
                        if (n % p) == 0 {
                            let mut count = 0;
                            while (n % p) == 0 {
                                n /= p;
                                count += 1;
                            }
                            if count > 1 {
                                return 0;
                            }
                            factors += 1;
                        }
                    }

                    if n > 1 {
                        factors += 1;
                    }

                    if (factors % 2 == 0) {1} else {-1}
                }

                fn factorization<'a, V, F>(self, primes: V, mut output: F)
                where
                    V: IntoIterator<Item = &'a Self>,
                    F: FnMut(Self, usize), Self: 'a
                {
                    let mut n = self;
                    for &p in primes {
                        if p * p > n {
                            break;
                        }
                        if (n % p) == 0 {
                            let mut count: usize = 0;
                            while (n % p) == 0 {
                                n /= p;
                                count += 1;
                            }
                            output(p, count);
                        }
                    }

                    if n > 1 {
                        output(n, 1);
                    }
                }

                fn divisors<'a, V: IntoIterator<Item = &'a Self>>(self, primes: V) -> Vec<Self> where Self: 'a {
                    let mut factor: HashMap<Self, usize> = HashMap::new();
                    self.factorization(primes, |p, d| {
                        factor.insert(p, d);
                    });

                    let mut result = Vec::new();
                    result.push(1);

                    for (p, exp) in factor {
                        let mut r = result.clone();
                        let mut f = p;
                        for _ in 0..exp {
                            result.iter().map(|i| i * f).for_each(|i| r.push(i));
                            f *= p;
                        }

                        result = r;
                    }

                    result.sort();
                    result
                }

                fn repunit_a(self, base: Self) -> Self {
                    let q = (base - 1) * self;
                    let mut p = base % q;
                    for k in 1.. {
                        if p % q == 1 {
                            return k;
                        }
                        p = (base * p) % q;
                    }

                    panic!()
                }

                fn count_factor(self, d: Self) -> usize {
                    let mut n = self;
                    let mut count = 0;
                    while n % d == 0 {
                        n/= d;
                        count += 1;
                    }
                    count
                }

            }
        )*
    }
}

impl_arithmetic!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_pgcd() {
        assert_eq!(u32::gcd(456753, 97643), 1);
        assert_eq!(u32::gcd(456755, 158665), 65);
    }

    #[test]
    fn test_ppcm() {
        assert_eq!(u64::lcm(456753u64, 97643u64), 44598733179u64);
        assert_eq!(u64::lcm(456755u64, 158665u64), 1114938955u64);
    }

    #[test]
    fn test_number_of_divisors() {
        let primes: Vec<u64> = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];
        assert_eq!(456753.number_of_divisors(&primes), 8);
        assert_eq!(3246999210.number_of_divisors(&primes), 640);
    }
    #[test]
    fn test_sum_of_divisors() {
        let primes: Vec<u64> = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];
        assert_eq!(456753.sum_of_divisors(&primes), 664416);
        assert_eq!(496.sum_of_divisors(&primes), 992);
        assert_eq!(3246999210u64.sum_of_divisors(&primes), 11708928000);
    }

    #[test]
    fn test_phi() {
        let primes: Vec<u64> = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];
        assert_eq!(3246999210.phi(&primes), 640120320);
        assert_eq!(496.phi(&primes), 240);
    }

    #[test]
    fn test_radical() {
        let primes: Vec<u64> = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];
        assert_eq!(456753.radical(&primes), 456753);
        assert_eq!(496.radical(&primes), 62);
        assert_eq!(3246999210u64.radical(&primes), 2454270);
    }

    #[test]
    fn test_mobius() {
        let primes: Vec<u64> = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];

        assert_eq!(3246999210.mobius(&primes), 0);
        assert_eq!(496.mobius(&primes), 0);
        assert_eq!(19.mobius(&primes), -1);
        assert_eq!(15.mobius(&primes), 1);
    }

    #[test]
    fn test_factorization() {
        let primes: Vec<u64> = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];
        let mut d: HashMap<u64, usize> = HashMap::new();
        3246999210.factorization(&primes, |p, c| {
            d.insert(p, c);
        });
        println!("{:?}", d);
        let expected = HashMap::from([(29, 1), (7, 3), (13, 1), (3, 4), (5, 1), (31, 1), (2, 1)]);
        assert_eq!(d, expected);
    }
    #[test]
    fn test_divisors() {
        let primes: Vec<u64> = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];
        let d = 3246210.divisors(&primes);
        println!("{:?}", d);
        let expected: Vec<u64> = vec![
            1, 2, 3, 5, 6, 9, 10, 11, 15, 18, 22, 27, 30, 33, 45, 54, 55, 66, 90, 99, 110, 135,
            165, 198, 270, 297, 330, 495, 594, 990, 1093, 1485, 2186, 2970, 3279, 5465, 6558, 9837,
            10930, 12023, 16395, 19674, 24046, 29511, 32790, 36069, 49185, 59022, 60115, 72138,
            98370, 108207, 120230, 147555, 180345, 216414, 295110, 324621, 360690, 541035, 649242,
            1082070, 1623105, 3246210,
        ];
        assert_eq!(d, expected);
    }

    #[test]
    fn test_repunit() {
        assert_eq!(11.repunit_a(10), 2);
        assert_eq!(3.repunit_a(10), 3);
        assert_eq!(7.repunit_a(10), 6);
    }
}
