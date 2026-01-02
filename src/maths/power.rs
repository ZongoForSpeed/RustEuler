trait ModArith: Sized + Copy {
    fn mod_mul(self, rhs: Self, m: Self) -> Self;
}

macro_rules! impl_modmul_widen {
    ($t:ty => $w:ty) => {
        impl ModArith for $t {
            #[inline]
            fn mod_mul(self, rhs: Self, m: Self) -> Self {
                (((self as $w) * (rhs as $w)) % (m as $w)) as $t
            }
        }
    };
}

impl_modmul_widen!(u8  => u16);
impl_modmul_widen!(u16 => u32);
impl_modmul_widen!(u32 => u64);
impl_modmul_widen!(u64 => u128);
impl_modmul_widen!(usize => u128);

impl ModArith for u128 {
    #[inline]
    fn mod_mul(self, rhs: Self, m: Self) -> Self {
        if m <= u64::MAX as u128 {
            let a = self % m;
            let b = rhs % m;
            return (a * b) % m;
        }

        let mut a = self % m;
        let mut b = rhs;
        let mut res = 0u128;

        while b > 0 {
            if b & 1 == 1 {
                res = (res + a) % m;
            }
            a = (a << 1) % m;
            b >>= 1;
        }
        res
    }
}

pub trait Power {
    fn power(self, exponent: Self) -> Self;

    fn power_mod(self, exponent: Self, modulo: Self) -> Self;
}

macro_rules! impl_power {
    ($($t:ty),*) => {
        $(
            impl Power for $t {
                #[inline]
                fn power(self: Self, mut exponent: Self) -> Self {
                    // TODO replace with pow(...)
                    // return _base.pow(_exposant as u32);
                    let mut base = self;

                    let mut result = 1;
                    while exponent > 0 {
                        if exponent & 1 == 1 {
                            result *= base;
                        }
                        exponent >>= 1;
                        base *= base;
                    }
                    result
                }

                #[inline]
                fn power_mod(self, mut exponent: Self, modulo: Self) -> Self {
                    let mut base = self % modulo;
                    let mut result: Self = 1;

                    while exponent > 0 {
                        if exponent & 1 == 1 {
                            result = result.mod_mul(base, modulo);
                        }
                        base = base.mod_mul(base, modulo);
                        exponent >>= 1;
                    }
                    result
                }
            }
        )*
    }
}

impl_power!(u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power() {
        assert_eq!(u32::power(2, 10), 1024);
    }

    #[test]
    fn test_power_mod() {
        assert_eq!(u32::power_mod(2, 10, 25), 24);
        assert_eq!(u64::power_mod(2, 10, 25), 24);
    }
}
