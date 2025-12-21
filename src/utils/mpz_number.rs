use gmp_mpfr_sys::gmp;
use gmp_mpfr_sys::gmp::mpz_t;
use gmp_mpfr_sys::mpc::free_str;
use num_traits::{One, Zero};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::ffi::{CStr, CString, c_char, c_int, c_long, c_ulong, c_void};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use std::ptr::null_mut;
use std::str::FromStr;

#[derive(Debug)]
pub(crate) struct MpzNumber {
    pub data: mpz_t,
}

impl MpzNumber {
    pub fn new() -> Self {
        let z = unsafe {
            let mut z = MaybeUninit::uninit();
            gmp::mpz_init(z.as_mut_ptr());
            z.assume_init()
        };
        Self { data: z }
    }

    pub fn from_u64(n: u64) -> Self {
        let z = unsafe {
            let mut z = MaybeUninit::uninit();
            gmp::mpz_init_set_ui(z.as_mut_ptr(), n as c_ulong);
            z.assume_init()
        };
        Self { data: z }
    }

    pub fn from_i64(n: i64) -> Self {
        let z = unsafe {
            let mut z = MaybeUninit::uninit();
            gmp::mpz_init_set_si(z.as_mut_ptr(), n as c_long);
            z.assume_init()
        };
        Self { data: z }
    }

    pub fn from_z(n: &MpzNumber) -> Self {
        let z = unsafe {
            let mut z = MaybeUninit::uninit();
            gmp::mpz_init_set(z.as_mut_ptr(), &n.data);
            z.assume_init()
        };
        Self { data: z }
    }

    pub fn from_str(n: String) -> Self {
        let z = unsafe {
            let mut z = MaybeUninit::uninit();
            let c_str = CString::new(n.as_str()).unwrap();
            let c_world: *const c_char = c_str.as_ptr() as *const c_char;
            gmp::mpz_init_set_str(z.as_mut_ptr(), c_world, 10);
            z.assume_init()
        };
        Self { data: z }
    }

    pub fn set_ui(&mut self, ui: u64) {
        unsafe {
            gmp::mpz_set_ui(&mut self.data, ui as c_ulong);
        }
    }

    pub fn set_u128(&mut self, n: u128) {
        unsafe {
            let my_num_ptr: *const u128 = &n;
            gmp::mpz_import(
                &mut self.data,
                1,
                -1,
                128 / 8,
                0,
                0,
                my_num_ptr as *const c_void,
            );
        }
    }

    pub fn set_si(&mut self, si: i64) {
        unsafe {
            gmp::mpz_set_si(&mut self.data, si as c_long);
        }
    }

    pub fn get_ui(&self) -> c_ulong {
        unsafe { gmp::mpz_get_ui(&self.data) }
    }

    pub fn get_u128(&self) -> u128 {
        let mut res = 0;
        unsafe {
            let my_num_ptr: *mut u128 = &mut res;
            gmp::mpz_export(
                my_num_ptr as *mut c_void,
                0 as *mut usize,
                -1,
                128 / 8,
                0,
                0,
                &self.data,
            );
        }
        res
    }

    pub fn get_si(&self) -> c_long {
        unsafe { gmp::mpz_get_si(&self.data) }
    }

    pub fn get_str(&self) -> String {
        unsafe {
            let char_ptr = gmp::mpz_get_str(null_mut(), 10, &self.data);
            let c_str = CStr::from_ptr(char_ptr);
            let string = c_str.to_string_lossy().into_owned();
            free_str(char_ptr);
            string
        }
    }

    pub fn to_string(&self) -> String {
        self.get_str()
    }

    // region Operations
    fn internal_add(lhs: &MpzNumber, rhs: &MpzNumber) -> MpzNumber {
        let mut rop = MpzNumber::new();
        unsafe {
            gmp::mpz_add(&mut rop.data, &lhs.data, &rhs.data);
        }
        rop
    }

    fn internal_add_ui(lhs: &MpzNumber, rhs: u64) -> MpzNumber {
        let mut rop = MpzNumber::new();
        unsafe {
            gmp::mpz_add_ui(&mut rop.data, &lhs.data, rhs);
        }
        rop
    }

    fn internal_add_assign(&mut self, rhs: &MpzNumber) {
        unsafe {
            gmp::mpz_add(&mut self.data, &self.data, &rhs.data);
        }
    }

    fn internal_mul(lhs: &MpzNumber, rhs: &MpzNumber) -> MpzNumber {
        let mut rop = MpzNumber::new();
        unsafe {
            gmp::mpz_mul(&mut rop.data, &lhs.data, &rhs.data);
        }
        rop
    }

    fn internal_mul_ui(lhs: &MpzNumber, rhs: u64) -> MpzNumber {
        let mut rop = MpzNumber::new();
        unsafe {
            gmp::mpz_mul_ui(&mut rop.data, &lhs.data, rhs);
        }
        rop
    }

    fn internal_mul_si(lhs: &MpzNumber, rhs: i64) -> MpzNumber {
        let mut rop = MpzNumber::new();
        unsafe {
            gmp::mpz_mul_si(&mut rop.data, &lhs.data, rhs);
        }
        rop
    }

    fn internal_mul_assign(&mut self, rhs: &MpzNumber) {
        unsafe {
            gmp::mpz_mul(&mut self.data, &self.data, &rhs.data);
        }
    }

    fn internal_sub(lhs: &MpzNumber, rhs: &MpzNumber) -> MpzNumber {
        let mut rop = MpzNumber::new();
        unsafe {
            gmp::mpz_sub(&mut rop.data, &lhs.data, &rhs.data);
        }
        rop
    }

    fn internal_ui_sub(lhs: u64, rhs: &MpzNumber) -> MpzNumber {
        let mut rop = MpzNumber::new();
        unsafe {
            gmp::mpz_ui_sub(&mut rop.data, lhs, &rhs.data);
        }
        rop
    }

    fn internal_sub_assign(&mut self, rhs: &MpzNumber) {
        unsafe {
            gmp::mpz_sub(&mut self.data, &self.data, &rhs.data);
        }
    }

    fn internal_div(lhs: &MpzNumber, rhs: &MpzNumber) -> MpzNumber {
        let mut rop = MpzNumber::new();
        unsafe {
            gmp::mpz_fdiv_q(&mut rop.data, &lhs.data, &rhs.data);
        }
        rop
    }

    fn internal_div_assign(&mut self, rhs: &MpzNumber) {
        unsafe {
            gmp::mpz_fdiv_q(&mut self.data, &self.data, &rhs.data);
        }
    }

    fn internal_mod(lhs: &MpzNumber, rhs: &MpzNumber) -> MpzNumber {
        let mut rop = MpzNumber::new();
        unsafe {
            gmp::mpz_mod(&mut rop.data, &lhs.data, &rhs.data);
        }
        rop
    }

    fn internal_mod_assign(&mut self, rhs: &MpzNumber) {
        unsafe {
            gmp::mpz_mod(&mut self.data, &self.data, &rhs.data);
        }
    }
    // endregion

    fn cmp(&self, other: &&MpzNumber) -> c_int {
        unsafe { gmp::mpz_cmp(&self.data, &other.data) }
    }

    fn clear(&mut self) {
        unsafe {
            gmp::mpz_clear(&mut self.data);
        }
    }

    pub fn binomial_ui(n: u64, k: u64) -> MpzNumber {
        let mut res = MpzNumber::new();
        unsafe {
            gmp::mpz_bin_uiui(&mut res.data, n, k);
        }
        res
    }

    pub fn factorial(n: u64) -> MpzNumber {
        let mut res = MpzNumber::new();
        unsafe {
            gmp::mpz_fac_ui(&mut res.data, n);
        }
        res
    }

    pub(crate) fn sign(&self) -> c_int {
        unsafe { gmp::mpz_sgn(&self.data) }
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.sign() == 0
    }

    pub(crate) fn loop_digits<F>(&self, base: u64, mut op: F)
    where
        F: FnMut(u64),
    {
        let mut copy = self.clone();
        unsafe {
            while !copy.is_zero() {
                let r = gmp::mpz_fdiv_q_ui(&mut copy.data, &copy.data, base) as u64;
                op(r);
            }
        }
    }

    pub(crate) fn extract_digits(&self, base: u64) -> VecDeque<u64> {
        let mut chiffres = VecDeque::new();
        self.loop_digits(base, |r| {
            chiffres.push_front(r);
        });
        chiffres
    }

    pub(crate) fn conversion<V>(list: V, base: u64) -> MpzNumber
    where
        V: IntoIterator<Item = u64>,
    {
        let mut z = MpzNumber::new();
        for d in list.into_iter() {
            z *= base;
            z += d;
        }
        z
    }

    pub(crate) fn invert(&self, base: u64) -> MpzNumber {
        let chiffres = self.extract_digits(base);
        let chiffres: Vec<u64> = chiffres.into_iter().rev().collect();
        Self::conversion(chiffres, base)
    }

    pub(crate) fn palindrome(&self, base: u64) -> bool {
        let chiffres = self.extract_digits(base);
        chiffres.iter().eq(chiffres.iter().rev())
    }

    pub(crate) fn sum_digits(&self, base: u64) -> u64 {
        let mut result: u64 = 0;
        self.loop_digits(base, |r| result += r);
        result
    }

    pub(crate) fn number_digits(&self, base: u64) -> u64 {
        let mut result: u64 = 0;
        self.loop_digits(base, |_r| result += 1);
        result
    }

    pub(crate) fn power_ui(base: u64, exponent: u64) -> MpzNumber {
        let mut res = MpzNumber::new();
        unsafe {
            gmp::mpz_ui_pow_ui(&mut res.data, base, exponent);
        }
        res
    }

    pub(crate) fn power_mod_ui(base: u64, exponent: u64, modulo: u64) -> MpzNumber {
        let mut res = MpzNumber::new();
        let z_base = MpzNumber::from_u64(base);
        let z_modulo = MpzNumber::from_u64(modulo);
        unsafe {
            gmp::mpz_powm_ui(&mut res.data, &z_base.data, exponent, &z_modulo.data);
        }
        res
    }

    pub(crate) fn sqrt(&self) -> MpzNumber {
        let mut res = MpzNumber::new();
        unsafe {
            gmp::mpz_sqrt(&mut res.data, &self.data);
        }
        res
    }
    
    fn hash<H: Hasher>(&self, state: &mut H) {
        // TODO implement better hash method
        self.to_string().hash(state);
    }
}

impl Drop for MpzNumber {
    fn drop(&mut self) {
        self.clear();
    }
}

impl Clone for MpzNumber {
    fn clone(&self) -> Self {
        MpzNumber::from_z(self)
    }
}

impl Display for MpzNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl FromStr for MpzNumber {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MpzNumber::from_str(s.to_string()))
    }
}

impl Zero for MpzNumber {
    fn zero() -> Self {
        MpzNumber::new()
    }

    fn is_zero(&self) -> bool {
        self.is_zero()
    }
}

impl One for MpzNumber {
    fn one() -> Self {
        MpzNumber::from_u64(1)
    }
}

impl PartialEq for MpzNumber {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(&other) == 0
    }

    fn ne(&self, other: &Self) -> bool {
        self.cmp(&other) != 0
    }
}

impl Eq for MpzNumber {}

impl Hash for MpzNumber {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash(state);
        // self.hash(state);
    }
}

impl PartialOrd for MpzNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cmp = self.cmp(&other);
        if cmp > 0 {
            Some(Ordering::Greater)
        } else if cmp < 0 {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }

    fn lt(&self, other: &Self) -> bool {
        self.cmp(&other) < 0
    }

    fn le(&self, other: &Self) -> bool {
        self.cmp(&other) <= 0
    }

    fn gt(&self, other: &Self) -> bool {
        self.cmp(&other) > 0
    }

    fn ge(&self, other: &Self) -> bool {
        self.cmp(&other) >= 0
    }
}

// region From
macro_rules! impl_from_unsigned {
    ($($t:ty),*) => {
        $(
            impl From<$t> for MpzNumber {
                fn from(value: $t) -> MpzNumber {
                    MpzNumber::from_u64(value as u64)
                }
            }

            impl Add<&MpzNumber> for $t {
                type Output = MpzNumber;

                fn add(self, rhs: &MpzNumber) -> MpzNumber {
                    MpzNumber::internal_add_ui(&rhs, self as u64)
                }
            }

            impl Mul<&MpzNumber> for $t {
                type Output = MpzNumber;

                fn mul(self, rhs: &MpzNumber) -> MpzNumber {
                    MpzNumber::internal_mul_ui(&rhs, self as u64)
                }
            }

            impl Sub<&MpzNumber> for $t {
                type Output = MpzNumber;

                fn sub(self, rhs: &MpzNumber) -> MpzNumber {
                    MpzNumber::internal_ui_sub(self as u64, &rhs)
                }
            }

            impl Div<&MpzNumber> for $t {
                type Output = MpzNumber;

                fn div(self, rhs: &MpzNumber) -> MpzNumber {
                    MpzNumber::internal_div(&MpzNumber::from_u64(self as u64), &rhs)
                }
            }

            impl Rem<&MpzNumber> for $t {
                type Output = MpzNumber;

                fn rem(self, rhs: &MpzNumber) -> MpzNumber {
                    MpzNumber::internal_mod(&MpzNumber::from_u64(self as u64), &rhs)
                }
            }
            impl Add<MpzNumber> for $t {
                type Output = MpzNumber;

                fn add(self, rhs: MpzNumber) -> MpzNumber {
                    MpzNumber::internal_add_ui(&rhs, self as u64)
                }
            }

            impl Mul<MpzNumber> for $t {
                type Output = MpzNumber;

                fn mul(self, rhs: MpzNumber) -> MpzNumber {
                    MpzNumber::internal_mul_ui(&rhs, self as u64)
                }
            }

            impl Sub<MpzNumber> for $t {
                type Output = MpzNumber;

                fn sub(self, rhs: MpzNumber) -> MpzNumber {
                    MpzNumber::internal_ui_sub(self as u64, &rhs)
                }
            }

            impl Div<MpzNumber> for $t {
                type Output = MpzNumber;

                fn div(self, rhs: MpzNumber) -> MpzNumber {
                    MpzNumber::internal_div(&MpzNumber::from_u64(self as u64), &rhs)
                }
            }

            impl Rem<MpzNumber> for $t {
                type Output = MpzNumber;

                fn rem(self, rhs: MpzNumber) -> MpzNumber {
                    MpzNumber::internal_mod(&MpzNumber::from_u64(self as u64), &rhs)
                }
            }
        )*
    }
}

impl_from_unsigned!(u8, u16, u32, u64, usize);

macro_rules! impl_from_signed {
    ($($t:ty),*) => {
        $(
            impl From<$t> for MpzNumber {
                fn from(value: $t) -> MpzNumber {
                    MpzNumber::from_i64(value as i64)
                }
            }

            impl Add<&MpzNumber> for $t {
                type Output = MpzNumber;

                fn add(self, rhs: &MpzNumber) -> MpzNumber {
                    MpzNumber::internal_add(&MpzNumber::from_i64(self as i64), &rhs)
                }
            }

            impl Mul<&MpzNumber> for $t {
                type Output = MpzNumber;

                fn mul(self, rhs: &MpzNumber) -> MpzNumber {
                    MpzNumber::internal_mul_si(&rhs, self as i64)
                }
            }

            impl Sub<&MpzNumber> for $t {
                type Output = MpzNumber;

                fn sub(self, rhs: &MpzNumber) -> MpzNumber {
                    MpzNumber::internal_sub(&MpzNumber::from_i64(self as i64), &rhs)
                }
            }

            impl Div<&MpzNumber> for $t {
                type Output = MpzNumber;

                fn div(self, rhs: &MpzNumber) -> MpzNumber {
                    MpzNumber::internal_div(&MpzNumber::from_i64(self as i64), &rhs)
                }
            }

            impl Rem<&MpzNumber> for $t {
                type Output = MpzNumber;

                fn rem(self, rhs: &MpzNumber) -> MpzNumber {
                    MpzNumber::internal_mod(&MpzNumber::from_i64(self as i64), &rhs)
                }
            }

            impl Add<MpzNumber> for $t {
                type Output = MpzNumber;

                fn add(self, rhs: MpzNumber) -> MpzNumber {
                    MpzNumber::internal_add(&MpzNumber::from_i64(self as i64), &rhs)
                }
            }

            impl Mul<MpzNumber> for $t {
                type Output = MpzNumber;

                fn mul(self, rhs: MpzNumber) -> MpzNumber {
                    MpzNumber::internal_mul_si(&rhs, self as i64)
                }
            }

            impl Sub<MpzNumber> for $t {
                type Output = MpzNumber;

                fn sub(self, rhs: MpzNumber) -> MpzNumber {
                    MpzNumber::internal_sub(&MpzNumber::from_i64(self as i64), &rhs)
                }
            }

            impl Div<MpzNumber> for $t {
                type Output = MpzNumber;

                fn div(self, rhs: MpzNumber) -> MpzNumber {
                    MpzNumber::internal_div(&MpzNumber::from_i64(self as i64), &rhs)
                }
            }

            impl Rem<MpzNumber> for $t {
                type Output = MpzNumber;

                fn rem(self, rhs: MpzNumber) -> MpzNumber {
                    MpzNumber::internal_mod(&MpzNumber::from_i64(self as i64), &rhs)
                }
            }
        )*
    }
}

impl_from_signed!(i8, i16, i32, i64, isize);
// endregion

// region Addition
impl<T> Add<T> for MpzNumber
where
    T: Into<MpzNumber>,
{
    type Output = MpzNumber;
    fn add(self, rhs: T) -> Self::Output {
        MpzNumber::internal_add(&self, &rhs.into())
    }
}

impl<T> Add<T> for &MpzNumber
where
    T: Into<MpzNumber>,
{
    type Output = MpzNumber;
    fn add(self, rhs: T) -> Self::Output {
        MpzNumber::internal_add(&self, &rhs.into())
    }
}

impl<'a, 'b> Add<&'b MpzNumber> for &'a MpzNumber {
    type Output = MpzNumber;

    fn add(self, rhs: &'b MpzNumber) -> MpzNumber {
        MpzNumber::internal_add(&self, &rhs)
    }
}

impl AddAssign<&MpzNumber> for MpzNumber {
    fn add_assign(&mut self, rhs: &MpzNumber) {
        self.internal_add_assign(&rhs);
    }
}

impl<T> AddAssign<T> for MpzNumber
where
    T: Into<MpzNumber>,
{
    fn add_assign(&mut self, rhs: T) {
        self.internal_add_assign(&rhs.into());
    }
}
// endregion

// region Multiplication
impl<T> Mul<T> for MpzNumber
where
    T: Into<MpzNumber>,
{
    type Output = MpzNumber;
    fn mul(self, rhs: T) -> Self::Output {
        MpzNumber::internal_mul(&self, &rhs.into())
    }
}

impl<T> Mul<T> for &MpzNumber
where
    T: Into<MpzNumber>,
{
    type Output = MpzNumber;
    fn mul(self, rhs: T) -> Self::Output {
        MpzNumber::internal_mul(&self, &rhs.into())
    }
}

impl<'a, 'b> Mul<&'b MpzNumber> for &'a MpzNumber {
    type Output = MpzNumber;

    fn mul(self, rhs: &'b MpzNumber) -> MpzNumber {
        MpzNumber::internal_mul(&self, &rhs)
    }
}

impl MulAssign<&MpzNumber> for MpzNumber {
    fn mul_assign(&mut self, rhs: &MpzNumber) {
        self.internal_mul_assign(&rhs);
    }
}

impl<T> MulAssign<T> for MpzNumber
where
    T: Into<MpzNumber>,
{
    fn mul_assign(&mut self, rhs: T) {
        self.internal_mul_assign(&rhs.into());
    }
}
// endregion

// region Substraction
impl<T> Sub<T> for MpzNumber
where
    T: Into<MpzNumber>,
{
    type Output = MpzNumber;
    fn sub(self, rhs: T) -> Self::Output {
        MpzNumber::internal_sub(&self, &rhs.into())
    }
}

impl<T> Sub<T> for &MpzNumber
where
    T: Into<MpzNumber>,
{
    type Output = MpzNumber;
    fn sub(self, rhs: T) -> Self::Output {
        MpzNumber::internal_sub(&self, &rhs.into())
    }
}

impl<'a, 'b> Sub<&'b MpzNumber> for &'a MpzNumber {
    type Output = MpzNumber;

    fn sub(self, rhs: &'b MpzNumber) -> MpzNumber {
        MpzNumber::internal_sub(&self, &rhs)
    }
}

impl SubAssign<&MpzNumber> for MpzNumber {
    fn sub_assign(&mut self, rhs: &MpzNumber) {
        self.internal_sub_assign(&rhs);
    }
}

impl<T> SubAssign<T> for MpzNumber
where
    T: Into<MpzNumber>,
{
    fn sub_assign(&mut self, rhs: T) {
        self.internal_sub_assign(&rhs.into());
    }
}
// endregion

// region Division
impl<T> Div<T> for MpzNumber
where
    T: Into<MpzNumber>,
{
    type Output = MpzNumber;
    fn div(self, rhs: T) -> Self::Output {
        MpzNumber::internal_div(&self, &rhs.into())
    }
}

impl<T> Div<T> for &MpzNumber
where
    T: Into<MpzNumber>,
{
    type Output = MpzNumber;
    fn div(self, rhs: T) -> Self::Output {
        MpzNumber::internal_div(&self, &rhs.into())
    }
}

impl<'a, 'b> Div<&'b MpzNumber> for &'a MpzNumber {
    type Output = MpzNumber;

    fn div(self, rhs: &'b MpzNumber) -> MpzNumber {
        MpzNumber::internal_div(&self, &rhs)
    }
}

impl DivAssign<&MpzNumber> for MpzNumber {
    fn div_assign(&mut self, rhs: &MpzNumber) {
        self.internal_div_assign(&rhs);
    }
}

impl<T> DivAssign<T> for MpzNumber
where
    T: Into<MpzNumber>,
{
    fn div_assign(&mut self, rhs: T) {
        self.internal_div_assign(&rhs.into());
    }
}
// endregion

// region Modulo
impl<T> Rem<T> for MpzNumber
where
    T: Into<MpzNumber>,
{
    type Output = MpzNumber;
    fn rem(self, rhs: T) -> Self::Output {
        MpzNumber::internal_mod(&self, &rhs.into())
    }
}

impl<T> Rem<T> for &MpzNumber
where
    T: Into<MpzNumber>,
{
    type Output = MpzNumber;
    fn rem(self, rhs: T) -> Self::Output {
        MpzNumber::internal_mod(&self, &rhs.into())
    }
}

impl<'a, 'b> Rem<&'b MpzNumber> for &'a MpzNumber {
    type Output = MpzNumber;

    fn rem(self, rhs: &'b MpzNumber) -> MpzNumber {
        MpzNumber::internal_mod(&self, &rhs)
    }
}

impl RemAssign<&MpzNumber> for MpzNumber {
    fn rem_assign(&mut self, rhs: &MpzNumber) {
        self.internal_mod_assign(&rhs);
    }
}

impl<T> RemAssign<T> for MpzNumber
where
    T: Into<MpzNumber>,
{
    fn rem_assign(&mut self, rhs: T) {
        self.internal_mod_assign(&rhs.into());
    }
}
// endregion

#[cfg(test)]
mod tests {
    use super::*;
    use gmp_mpfr_sys::gmp;
    use std::ffi::CStr;
    use std::ptr::null_mut;

    #[test]
    fn test_gmp_dummy() {
        unsafe {
            let mut z = {
                let mut z = MaybeUninit::uninit();
                gmp::mpz_init(z.as_mut_ptr());
                z.assume_init()
            };
            gmp::mpz_set_ui(&mut z, 15);
            let u = gmp::mpz_get_ui(&z);
            assert_eq!(u, 15);

            let char_ptr = gmp::mpz_get_str(null_mut(), 10, &z);
            let c_str = CStr::from_ptr(char_ptr);
            let result = c_str.to_str();

            assert_eq!(result.is_ok(), true);
            assert_eq!(result.ok(), Some("15"));

            free_str(char_ptr);
            gmp::mpz_clear(&mut z);
        }
    }

    #[test]
    fn test_init() {
        let mpz: MpzNumber = MpzNumber::new();
        assert_eq!(mpz.get_ui(), 0);
        assert_eq!(mpz.get_str(), "0");
    }

    #[test]
    fn test_set() {
        let mut mpz: MpzNumber = MpzNumber::new();
        mpz.set_ui(15);
        assert_eq!(mpz.get_ui(), 15);
        assert_eq!(mpz.get_str(), "15");

        mpz.set_si(-42);
        assert_eq!(mpz.get_si(), -42);

        let z = MpzNumber::from_z(&mpz);
        assert_eq!(z, mpz);
    }

    #[test]
    fn test_init_ui() {
        let mpz: MpzNumber = MpzNumber::from_u64(165);
        assert_eq!(mpz.get_ui(), 165);
        assert_eq!(mpz.get_str(), "165");
    }

    #[test]
    fn test_init_u128() {
        let mut mpz: MpzNumber = MpzNumber::new();
        mpz.set_u128(123456789098760000066600005432123456789);
        assert_eq!(mpz.get_u128(), 123456789098760000066600005432123456789);
    }

    #[test]
    fn test_bin() {
        let bin = MpzNumber::binomial_ui(40, 20);
        assert_eq!(bin.get_u128(), 137846528820);

        let fac = MpzNumber::factorial(20);
        assert_eq!(fac.get_u128(), 2432902008176640000);

        assert_eq!(fac > bin, true);
        assert_eq!(fac < bin, false);
        assert_eq!(fac == bin, false);
        assert_eq!(fac != bin, true);
        assert_eq!(fac.is_zero(), false);
        assert_eq!(fac.sign(), 1);
    }

    #[test]
    fn test_init_str() {
        let mpz: MpzNumber = MpzNumber::from_str("123456".to_string());
        assert_eq!(mpz.get_ui(), 123456);
        assert_eq!(mpz.get_str(), "123456");
    }

    #[test]
    fn test_addition() {
        let n = MpzNumber::from_u64(12345678);
        let m = MpzNumber::from_u64(987654321);
        let r: MpzNumber = n + m;
        assert_eq!(r.get_ui(), 999999999);
        assert_eq!(r.get_str(), "999999999");

        let nn = MpzNumber::from_str("12345678901234567890".to_string());
        let mm = MpzNumber::from_str("123456789098765432123456788765".to_string());
        let mut rr: MpzNumber = nn + mm;
        assert_eq!(rr.get_str(), "123456789111111111024691356655");

        rr += &r;
        assert_eq!(rr.get_str(), "123456789111111111025691356654");
        assert_eq!(rr.get_u128(), 123456789111111111025691356654);

        rr = &r + 1234567890;
        assert_eq!(rr.get_str(), "2234567889");
        rr = 12345566 + &r;
        assert_eq!(rr.get_str(), "1012345565");
    }

    #[test]
    fn test_multiplication() {
        let n = MpzNumber::from_u64(12345678);
        let m = MpzNumber::from_u64(987654321);
        let r = n * m;
        assert_eq!(r.get_ui(), 12193262222374638);
        assert_eq!(r.get_str(), "12193262222374638");

        let nn = MpzNumber::from_str("12345678901234567890".to_string());
        let mm = MpzNumber::from_str("123456789098765432123456788765".to_string());
        let mut rr = nn * mm;
        assert_eq!(
            rr.get_str(),
            "1524157876390794199039780513259411825711781755850"
        );

        rr *= &r;
        assert_eq!(
            rr.get_str(),
            "18584456655130624074223798764637621592317996271475642992148132300"
        );

        rr = &r * 1234567890;
        assert_eq!(rr.get_str(), "15053410014093767625173820");
        rr = 12345566 * &r;
        assert_eq!(rr.get_str(), "150532723521632770155108");
    }

    #[test]
    fn test_subtraction() {
        let n = MpzNumber::from_u64(12345678);
        let m = MpzNumber::from_u64(987654321);
        let r = n - m;
        assert_eq!(r.get_si(), -975308643);
        assert_eq!(r.get_str(), "-975308643");

        let nn = MpzNumber::from_str("12345678901234567890".to_string());
        let mm = MpzNumber::from_str("123456789098765432123456788765".to_string());
        let mut rr = nn - mm;
        assert_eq!(rr.get_str(), "-123456789086419753222222220875");

        rr -= &r;
        assert_eq!(rr.get_str(), "-123456789086419753221246912232");

        rr = &r - 1234567890;
        assert_eq!(rr.get_str(), "-2209876533");
        rr = 12345566 - &r;
        assert_eq!(rr.get_str(), "987654209");
    }

    #[test]
    fn test_division() {
        let n = MpzNumber::from_u64(12345678);
        let m = MpzNumber::from_u64(987654321);
        let r = m / n;
        assert_eq!(r.get_si(), 80);
        assert_eq!(r.get_str(), "80");

        let nn = MpzNumber::from_str("12345678901234567890".to_string());
        let mm = MpzNumber::from_str("123456789098765432123456788765".to_string());
        let mut rr = mm / nn;
        assert_eq!(rr.get_str(), "10000000007");

        rr /= &r;
        assert_eq!(rr.get_str(), "125000000");

        rr = &r / 10;
        assert_eq!(rr.get_str(), "8");
        rr = 12345566123456i64 / &r;
        assert_eq!(rr.get_str(), "154319576543");
    }

    #[test]
    fn test_mod() {
        let n = MpzNumber::from_u64(12345678);
        let m = MpzNumber::from_u64(987654321);
        let r = m % n;
        assert_eq!(r.get_si(), 81);
        assert_eq!(r.get_str(), "81");

        let nn = MpzNumber::from_str("12345678901234567890".to_string());
        let mm = MpzNumber::from_str("123456789098765432123456788765".to_string());
        let mut rr = mm % nn;
        assert_eq!(rr.get_str(), "914814813535");

        rr %= &r;
        assert_eq!(rr.get_str(), "52");

        rr = &r % 10;
        assert_eq!(rr.get_str(), "1");
        rr = 12345566123456i64 % &r;
        assert_eq!(rr.get_str(), "71");
    }

    #[test]
    fn test_puissance() {
        let p = MpzNumber::power_ui(11, 11);
        assert_eq!(p.get_ui(), 285311670611);
    }

    #[test]
    fn test_puissance_m() {
        let p = MpzNumber::power_mod_ui(11, 1111, 123456789);
        assert_eq!(p.get_ui(), 116094638);
    }
}
