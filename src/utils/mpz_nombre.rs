use gmp_mpfr_sys::gmp;
use gmp_mpfr_sys::gmp::mpz_t;
use gmp_mpfr_sys::mpc::free_str;
use num_traits::{One, Zero};
use std::cmp::Ordering;
use std::ffi::{CStr, CString, c_char, c_int, c_long, c_ulong, c_void};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use std::ptr::null_mut;
use std::str::FromStr;

#[derive(Debug)]
pub(crate) struct MpzNombre {
    pub data: mpz_t,
}

impl MpzNombre {
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

    pub fn from_z(n: &MpzNombre) -> Self {
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
    fn internal_add(lhs: &MpzNombre, rhs: &MpzNombre) -> MpzNombre {
        let mut rop = MpzNombre::new();
        unsafe {
            gmp::mpz_add(&mut rop.data, &lhs.data, &rhs.data);
        }
        rop
    }

    fn internal_add_ui(lhs: &MpzNombre, rhs: u64) -> MpzNombre {
        let mut rop = MpzNombre::new();
        unsafe {
            gmp::mpz_add_ui(&mut rop.data, &lhs.data, rhs);
        }
        rop
    }

    fn internal_add_assign(&mut self, rhs: &MpzNombre) {
        unsafe {
            gmp::mpz_add(&mut self.data, &self.data, &rhs.data);
        }
    }

    fn internal_mul(lhs: &MpzNombre, rhs: &MpzNombre) -> MpzNombre {
        let mut rop = MpzNombre::new();
        unsafe {
            gmp::mpz_mul(&mut rop.data, &lhs.data, &rhs.data);
        }
        rop
    }

    fn internal_mul_ui(lhs: &MpzNombre, rhs: u64) -> MpzNombre {
        let mut rop = MpzNombre::new();
        unsafe {
            gmp::mpz_mul_ui(&mut rop.data, &lhs.data, rhs);
        }
        rop
    }

    fn internal_mul_si(lhs: &MpzNombre, rhs: i64) -> MpzNombre {
        let mut rop = MpzNombre::new();
        unsafe {
            gmp::mpz_mul_si(&mut rop.data, &lhs.data, rhs);
        }
        rop
    }

    fn internal_mul_assign(&mut self, rhs: &MpzNombre) {
        unsafe {
            gmp::mpz_mul(&mut self.data, &self.data, &rhs.data);
        }
    }

    fn internal_sub(lhs: &MpzNombre, rhs: &MpzNombre) -> MpzNombre {
        let mut rop = MpzNombre::new();
        unsafe {
            gmp::mpz_sub(&mut rop.data, &lhs.data, &rhs.data);
        }
        rop
    }

    fn internal_ui_sub(lhs: u64, rhs: &MpzNombre) -> MpzNombre {
        let mut rop = MpzNombre::new();
        unsafe {
            gmp::mpz_ui_sub(&mut rop.data, lhs, &rhs.data);
        }
        rop
    }

    fn internal_sub_assign(&mut self, rhs: &MpzNombre) {
        unsafe {
            gmp::mpz_sub(&mut self.data, &self.data, &rhs.data);
        }
    }

    fn internal_div(lhs: &MpzNombre, rhs: &MpzNombre) -> MpzNombre {
        let mut rop = MpzNombre::new();
        unsafe {
            gmp::mpz_fdiv_q(&mut rop.data, &lhs.data, &rhs.data);
        }
        rop
    }

    fn internal_div_assign(&mut self, rhs: &MpzNombre) {
        unsafe {
            gmp::mpz_fdiv_q(&mut self.data, &self.data, &rhs.data);
        }
    }

    fn internal_mod(lhs: &MpzNombre, rhs: &MpzNombre) -> MpzNombre {
        let mut rop = MpzNombre::new();
        unsafe {
            gmp::mpz_mod(&mut rop.data, &lhs.data, &rhs.data);
        }
        rop
    }

    fn internal_mod_assign(&mut self, rhs: &MpzNombre) {
        unsafe {
            gmp::mpz_mod(&mut self.data, &self.data, &rhs.data);
        }
    }
    // endregion

    fn cmp(&self, other: &&MpzNombre) -> c_int {
        unsafe { gmp::mpz_cmp(&self.data, &other.data) }
    }

    fn clear(&mut self) {
        unsafe {
            gmp::mpz_clear(&mut self.data);
        }
    }

    pub fn binomial_ui(n: u64, k: u64) -> MpzNombre {
        let mut res = MpzNombre::new();
        unsafe {
            gmp::mpz_bin_uiui(&mut res.data, n, k);
        }
        res
    }

    pub fn factorial(n: u64) -> MpzNombre {
        let mut res = MpzNombre::new();
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

    pub(crate) fn boucle_chiffre<F>(&self, base: u64, mut op: F)
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

    pub(crate) fn somme_chiffre(&self, base: u64) -> u64 {
        let mut result: u64 = 0;
        self.boucle_chiffre(base, |r| result += r);
        result
    }

    pub(crate) fn nombre_chiffre(&self, base: u64) -> u64 {
        let mut result: u64 = 0;
        self.boucle_chiffre(base, |_r| result += 1);
        result
    }

    pub(crate) fn puissance_ui(base: u64, exposant: u64) -> MpzNombre {
        let mut res = MpzNombre::new();
        unsafe {
            gmp::mpz_ui_pow_ui(&mut res.data, base, exposant);
        }
        res
    }

    pub(crate) fn puissance_m_ui(base: u64, exposant: u64, modulo: u64) -> MpzNombre {
        let mut res = MpzNombre::new();
        let z_base = MpzNombre::from_u64(base);
        let z_modulo = MpzNombre::from_u64(modulo);
        unsafe {
            //     pub fn mpz_powm_ui(rop: mpz_ptr, base: mpz_srcptr, exp: c_ulong, modu: mpz_srcptr);
            gmp::mpz_powm_ui(&mut res.data, &z_base.data, exposant, &z_modulo.data);
        }
        res
    }

    fn hash<H: Hasher>(&self, state: &mut H) {
        // TODO implement better hash method
        self.to_string().hash(state);
    }
}

impl Drop for MpzNombre {
    fn drop(&mut self) {
        self.clear();
    }
}

impl Clone for MpzNombre {
    fn clone(&self) -> Self {
        MpzNombre::from_z(self)
    }
}

impl Display for MpzNombre {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl FromStr for MpzNombre {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MpzNombre::from_str(s.to_string()))
    }
}

impl Zero for MpzNombre {
    fn zero() -> Self {
        MpzNombre::new()
    }

    fn is_zero(&self) -> bool {
        self.is_zero()
    }
}

impl One for MpzNombre {
    fn one() -> Self {
        MpzNombre::from_u64(1)
    }
}

impl PartialEq for MpzNombre {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(&other) == 0
    }

    fn ne(&self, other: &Self) -> bool {
        self.cmp(&other) != 0
    }
}

impl Eq for MpzNombre {}

impl Hash for MpzNombre {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash(state);
        // self.hash(state);
    }
}

impl PartialOrd for MpzNombre {
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
            impl From<$t> for MpzNombre {
                fn from(value: $t) -> MpzNombre {
                    MpzNombre::from_u64(value as u64)
                }
            }

            impl Add<&MpzNombre> for $t {
                type Output = MpzNombre;

                fn add(self, rhs: &MpzNombre) -> MpzNombre {
                    MpzNombre::internal_add_ui(&rhs, self as u64)
                }
            }

            impl Mul<&MpzNombre> for $t {
                type Output = MpzNombre;

                fn mul(self, rhs: &MpzNombre) -> MpzNombre {
                    MpzNombre::internal_mul_ui(&rhs, self as u64)
                }
            }

            impl Sub<&MpzNombre> for $t {
                type Output = MpzNombre;

                fn sub(self, rhs: &MpzNombre) -> MpzNombre {
                    MpzNombre::internal_ui_sub(self as u64, &rhs)
                }
            }

            impl Div<&MpzNombre> for $t {
                type Output = MpzNombre;

                fn div(self, rhs: &MpzNombre) -> MpzNombre {
                    MpzNombre::internal_div(&MpzNombre::from_u64(self as u64), &rhs)
                }
            }

            impl Rem<&MpzNombre> for $t {
                type Output = MpzNombre;

                fn rem(self, rhs: &MpzNombre) -> MpzNombre {
                    MpzNombre::internal_mod(&MpzNombre::from_u64(self as u64), &rhs)
                }
            }
        )*
    }
}

impl_from_unsigned!(u8, u16, u32, u64, usize);

macro_rules! impl_from_signed {
    ($($t:ty),*) => {
        $(
            impl From<$t> for MpzNombre {
                fn from(value: $t) -> MpzNombre {
                    MpzNombre::from_i64(value as i64)
                }
            }

            impl Add<&MpzNombre> for $t {
                type Output = MpzNombre;

                fn add(self, rhs: &MpzNombre) -> MpzNombre {
                    MpzNombre::internal_add(&MpzNombre::from_i64(self as i64), &rhs)
                }
            }

            impl Mul<&MpzNombre> for $t {
                type Output = MpzNombre;

                fn mul(self, rhs: &MpzNombre) -> MpzNombre {
                    MpzNombre::internal_mul_si(&rhs, self as i64)
                }
            }

            impl Sub<&MpzNombre> for $t {
                type Output = MpzNombre;

                fn sub(self, rhs: &MpzNombre) -> MpzNombre {
                    MpzNombre::internal_sub(&MpzNombre::from_i64(self as i64), &rhs)
                }
            }

            impl Div<&MpzNombre> for $t {
                type Output = MpzNombre;

                fn div(self, rhs: &MpzNombre) -> MpzNombre {
                    MpzNombre::internal_div(&MpzNombre::from_i64(self as i64), &rhs)
                }
            }

            impl Rem<&MpzNombre> for $t {
                type Output = MpzNombre;

                fn rem(self, rhs: &MpzNombre) -> MpzNombre {
                    MpzNombre::internal_mod(&MpzNombre::from_i64(self as i64), &rhs)
                }
            }
        )*
    }
}

impl_from_signed!(i8, i16, i32, i64, isize);
// endregion

// region Addition
impl<T> Add<T> for MpzNombre
where
    T: Into<MpzNombre>,
{
    type Output = MpzNombre;
    fn add(self, rhs: T) -> Self::Output {
        MpzNombre::internal_add(&self, &rhs.into())
    }
}

impl<T> Add<T> for &MpzNombre
where
    T: Into<MpzNombre>,
{
    type Output = MpzNombre;
    fn add(self, rhs: T) -> Self::Output {
        MpzNombre::internal_add(&self, &rhs.into())
    }
}

impl<'a, 'b> Add<&'b MpzNombre> for &'a MpzNombre {
    type Output = MpzNombre;

    fn add(self, rhs: &'b MpzNombre) -> MpzNombre {
        MpzNombre::internal_add(&self, &rhs)
    }
}

impl AddAssign<&MpzNombre> for MpzNombre {
    fn add_assign(&mut self, rhs: &MpzNombre) {
        self.internal_add_assign(&rhs);
    }
}

impl<T> AddAssign<T> for MpzNombre
where
    T: Into<MpzNombre>,
{
    fn add_assign(&mut self, rhs: T) {
        self.internal_add_assign(&rhs.into());
    }
}
// endregion

// region Multiplication
impl<T> Mul<T> for MpzNombre
where
    T: Into<MpzNombre>,
{
    type Output = MpzNombre;
    fn mul(self, rhs: T) -> Self::Output {
        MpzNombre::internal_mul(&self, &rhs.into())
    }
}

impl<T> Mul<T> for &MpzNombre
where
    T: Into<MpzNombre>,
{
    type Output = MpzNombre;
    fn mul(self, rhs: T) -> Self::Output {
        MpzNombre::internal_mul(&self, &rhs.into())
    }
}

impl<'a, 'b> Mul<&'b MpzNombre> for &'a MpzNombre {
    type Output = MpzNombre;

    fn mul(self, rhs: &'b MpzNombre) -> MpzNombre {
        MpzNombre::internal_mul(&self, &rhs)
    }
}

impl MulAssign<&MpzNombre> for MpzNombre {
    fn mul_assign(&mut self, rhs: &MpzNombre) {
        self.internal_mul_assign(&rhs);
    }
}

impl<T> MulAssign<T> for MpzNombre
where
    T: Into<MpzNombre>,
{
    fn mul_assign(&mut self, rhs: T) {
        self.internal_mul_assign(&rhs.into());
    }
}
// endregion

// region Substraction
impl<T> Sub<T> for MpzNombre
where
    T: Into<MpzNombre>,
{
    type Output = MpzNombre;
    fn sub(self, rhs: T) -> Self::Output {
        MpzNombre::internal_sub(&self, &rhs.into())
    }
}

impl<T> Sub<T> for &MpzNombre
where
    T: Into<MpzNombre>,
{
    type Output = MpzNombre;
    fn sub(self, rhs: T) -> Self::Output {
        MpzNombre::internal_sub(&self, &rhs.into())
    }
}

impl<'a, 'b> Sub<&'b MpzNombre> for &'a MpzNombre {
    type Output = MpzNombre;

    fn sub(self, rhs: &'b MpzNombre) -> MpzNombre {
        MpzNombre::internal_sub(&self, &rhs)
    }
}

impl SubAssign<&MpzNombre> for MpzNombre {
    fn sub_assign(&mut self, rhs: &MpzNombre) {
        self.internal_sub_assign(&rhs);
    }
}

impl<T> SubAssign<T> for MpzNombre
where
    T: Into<MpzNombre>,
{
    fn sub_assign(&mut self, rhs: T) {
        self.internal_sub_assign(&rhs.into());
    }
}
// endregion

// region Division
impl<T> Div<T> for MpzNombre
where
    T: Into<MpzNombre>,
{
    type Output = MpzNombre;
    fn div(self, rhs: T) -> Self::Output {
        MpzNombre::internal_div(&self, &rhs.into())
    }
}

impl<T> Div<T> for &MpzNombre
where
    T: Into<MpzNombre>,
{
    type Output = MpzNombre;
    fn div(self, rhs: T) -> Self::Output {
        MpzNombre::internal_div(&self, &rhs.into())
    }
}

impl<'a, 'b> Div<&'b MpzNombre> for &'a MpzNombre {
    type Output = MpzNombre;

    fn div(self, rhs: &'b MpzNombre) -> MpzNombre {
        MpzNombre::internal_div(&self, &rhs)
    }
}

impl DivAssign<&MpzNombre> for MpzNombre {
    fn div_assign(&mut self, rhs: &MpzNombre) {
        self.internal_div_assign(&rhs);
    }
}

impl<T> DivAssign<T> for MpzNombre
where
    T: Into<MpzNombre>,
{
    fn div_assign(&mut self, rhs: T) {
        self.internal_div_assign(&rhs.into());
    }
}
// endregion

// region Modulo
impl<T> Rem<T> for MpzNombre
where
    T: Into<MpzNombre>,
{
    type Output = MpzNombre;
    fn rem(self, rhs: T) -> Self::Output {
        MpzNombre::internal_mod(&self, &rhs.into())
    }
}

impl<T> Rem<T> for &MpzNombre
where
    T: Into<MpzNombre>,
{
    type Output = MpzNombre;
    fn rem(self, rhs: T) -> Self::Output {
        MpzNombre::internal_mod(&self, &rhs.into())
    }
}

impl<'a, 'b> Rem<&'b MpzNombre> for &'a MpzNombre {
    type Output = MpzNombre;

    fn rem(self, rhs: &'b MpzNombre) -> MpzNombre {
        MpzNombre::internal_mod(&self, &rhs)
    }
}

impl RemAssign<&MpzNombre> for MpzNombre {
    fn rem_assign(&mut self, rhs: &MpzNombre) {
        self.internal_mod_assign(&rhs);
    }
}

impl<T> RemAssign<T> for MpzNombre
where
    T: Into<MpzNombre>,
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
        let mpz: MpzNombre = MpzNombre::new();
        assert_eq!(mpz.get_ui(), 0);
        assert_eq!(mpz.get_str(), "0");
    }

    #[test]
    fn test_set() {
        let mut mpz: MpzNombre = MpzNombre::new();
        mpz.set_ui(15);
        assert_eq!(mpz.get_ui(), 15);
        assert_eq!(mpz.get_str(), "15");

        mpz.set_si(-42);
        assert_eq!(mpz.get_si(), -42);

        let z = MpzNombre::from_z(&mpz);
        assert_eq!(z, mpz);
    }

    #[test]
    fn test_init_ui() {
        let mpz: MpzNombre = MpzNombre::from_u64(165);
        assert_eq!(mpz.get_ui(), 165);
        assert_eq!(mpz.get_str(), "165");
    }

    #[test]
    fn test_init_u128() {
        let mut mpz: MpzNombre = MpzNombre::new();
        mpz.set_u128(123456789098760000066600005432123456789);
        assert_eq!(mpz.get_u128(), 123456789098760000066600005432123456789);
    }

    #[test]
    fn test_bin() {
        let bin = MpzNombre::binomial_ui(40, 20);
        assert_eq!(bin.get_u128(), 137846528820);

        let fac = MpzNombre::factorial(20);
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
        let mpz: MpzNombre = MpzNombre::from_str("123456".to_string());
        assert_eq!(mpz.get_ui(), 123456);
        assert_eq!(mpz.get_str(), "123456");
    }

    #[test]
    fn test_addition() {
        let n = MpzNombre::from_u64(12345678);
        let m = MpzNombre::from_u64(987654321);
        let r: MpzNombre = n + m;
        assert_eq!(r.get_ui(), 999999999);
        assert_eq!(r.get_str(), "999999999");

        let nn = MpzNombre::from_str("12345678901234567890".to_string());
        let mm = MpzNombre::from_str("123456789098765432123456788765".to_string());
        let mut rr: MpzNombre = nn + mm;
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
        let n = MpzNombre::from_u64(12345678);
        let m = MpzNombre::from_u64(987654321);
        let r = n * m;
        assert_eq!(r.get_ui(), 12193262222374638);
        assert_eq!(r.get_str(), "12193262222374638");

        let nn = MpzNombre::from_str("12345678901234567890".to_string());
        let mm = MpzNombre::from_str("123456789098765432123456788765".to_string());
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
        let n = MpzNombre::from_u64(12345678);
        let m = MpzNombre::from_u64(987654321);
        let r = n - m;
        assert_eq!(r.get_si(), -975308643);
        assert_eq!(r.get_str(), "-975308643");

        let nn = MpzNombre::from_str("12345678901234567890".to_string());
        let mm = MpzNombre::from_str("123456789098765432123456788765".to_string());
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
        let n = MpzNombre::from_u64(12345678);
        let m = MpzNombre::from_u64(987654321);
        let r = m / n;
        assert_eq!(r.get_si(), 80);
        assert_eq!(r.get_str(), "80");

        let nn = MpzNombre::from_str("12345678901234567890".to_string());
        let mm = MpzNombre::from_str("123456789098765432123456788765".to_string());
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
        let n = MpzNombre::from_u64(12345678);
        let m = MpzNombre::from_u64(987654321);
        let r = m % n;
        assert_eq!(r.get_si(), 81);
        assert_eq!(r.get_str(), "81");

        let nn = MpzNombre::from_str("12345678901234567890".to_string());
        let mm = MpzNombre::from_str("123456789098765432123456788765".to_string());
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
        let p = MpzNombre::puissance_ui(11, 11);
        assert_eq!(p.get_ui(), 285311670611);
    }

    #[test]
    fn test_puissance_m() {
        let p = MpzNombre::puissance_m_ui(11, 1111, 123456789);
        assert_eq!(p.get_ui(), 116094638);
    }

}
