use crate::utils::mpz_number::MpzNumber;
use gmp_mpfr_sys::gmp;
use gmp_mpfr_sys::mpc;
use num_traits::{One, Zero};
use std::cmp::Ordering;
use std::ffi::{CStr, CString, c_char, c_int};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::ptr::null_mut;
use std::str::FromStr;

#[derive(Debug)]
pub(crate) struct MpqFraction {
    pub data: gmp::mpq_t,
}

impl MpqFraction {
    pub fn new() -> Self {
        unsafe {
            let mut z = MaybeUninit::uninit();
            gmp::mpq_init(z.as_mut_ptr());
            MpqFraction {
                data: z.assume_init(),
            }
        }
    }

    fn clear(&mut self) {
        unsafe {
            gmp::mpq_clear(&mut self.data);
        }
    }

    pub fn from_u64(n: u64, d: u64) -> Self {
        let mut z = MpqFraction::new();
        unsafe {
            gmp::mpq_set_ui(&mut z.data, n, d);
            gmp::mpq_canonicalize(&mut z.data);
        }
        z
    }

    pub fn from_i64(n: i64, d: u64) -> Self {
        let mut z = MpqFraction::new();
        unsafe {
            gmp::mpq_set_si(&mut z.data, n, d);
            gmp::mpq_canonicalize(&mut z.data);
        }
        z
    }

    pub fn from_z(n: &MpzNumber) -> Self {
        let mut z = MpqFraction::new();
        unsafe {
            gmp::mpq_set_z(&mut z.data, &n.data);
            gmp::mpq_canonicalize(&mut z.data);
        };
        z
    }

    pub fn from_zz(n: &MpzNumber, d: &MpzNumber) -> Self {
        let mut z = MpqFraction::new();
        unsafe {
            gmp::mpq_set_num(&mut z.data, &n.data);
            gmp::mpq_set_den(&mut z.data, &d.data);
            gmp::mpq_canonicalize(&mut z.data);
        };
        z
    }

    pub fn from_q(q: &MpqFraction) -> Self {
        let mut z = MpqFraction::new();
        unsafe {
            gmp::mpq_set(&mut z.data, &q.data);
            gmp::mpq_canonicalize(&mut z.data);
        }
        z
    }

    pub fn from_str(n: String) -> Self {
        let mut z = MpqFraction::new();
        unsafe {
            let c_str = CString::new(n.as_str()).unwrap();
            let c_world: *const c_char = c_str.as_ptr() as *const c_char;
            gmp::mpq_set_str(&mut z.data, c_world, 10);
        }
        z
    }

    pub fn numerator(&self) -> MpzNumber {
        let mut n = MpzNumber::new();
        unsafe {
            gmp::mpq_get_num(&mut n.data, &self.data);
        }
        n
    }

    pub fn denominator(&self) -> MpzNumber {
        let mut d = MpzNumber::new();
        unsafe {
            gmp::mpq_get_den(&mut d.data, &self.data);
        }
        d
    }

    fn cmp(&self, other: &&MpqFraction) -> c_int {
        unsafe { gmp::mpq_cmp(&self.data, &other.data) }
    }

    pub fn get_str(&self) -> String {
        unsafe {
            let char_ptr = gmp::mpq_get_str(null_mut(), 10, &self.data);
            let c_str = CStr::from_ptr(char_ptr);
            let string = c_str.to_string_lossy().into_owned();
            mpc::free_str(char_ptr);
            string
        }
    }

    pub fn get_f(&self) -> f64 {
        unsafe { gmp::mpq_get_d(&self.data) }
    }

    pub fn to_string(&self) -> String {
        self.get_str()
    }

    // region Operations
    fn internal_add(lhs: &MpqFraction, rhs: &MpqFraction) -> MpqFraction {
        let mut rop = MpqFraction::new();
        unsafe {
            gmp::mpq_add(&mut rop.data, &lhs.data, &rhs.data);
            gmp::mpq_canonicalize(&mut rop.data);
        }
        rop
    }

    fn internal_add_assign(&mut self, rhs: &MpqFraction) {
        unsafe {
            gmp::mpq_add(&mut self.data, &self.data, &rhs.data);
            gmp::mpq_canonicalize(&mut self.data);
        }
    }

    fn internal_mul(lhs: &MpqFraction, rhs: &MpqFraction) -> MpqFraction {
        let mut rop = MpqFraction::new();
        unsafe {
            gmp::mpq_mul(&mut rop.data, &lhs.data, &rhs.data);
            gmp::mpq_canonicalize(&mut rop.data);
        }
        rop
    }

    fn internal_mul_assign(&mut self, rhs: &MpqFraction) {
        unsafe {
            gmp::mpq_mul(&mut self.data, &self.data, &rhs.data);
            gmp::mpq_canonicalize(&mut self.data);
        }
    }

    fn internal_sub(lhs: &MpqFraction, rhs: &MpqFraction) -> MpqFraction {
        let mut rop = MpqFraction::new();
        unsafe {
            gmp::mpq_sub(&mut rop.data, &lhs.data, &rhs.data);
            gmp::mpq_canonicalize(&mut rop.data);
        }
        rop
    }

    fn internal_sub_assign(&mut self, rhs: &MpqFraction) {
        unsafe {
            gmp::mpq_sub(&mut self.data, &self.data, &rhs.data);
            gmp::mpq_canonicalize(&mut self.data);
        }
    }

    fn internal_div(lhs: &MpqFraction, rhs: &MpqFraction) -> MpqFraction {
        let mut rop = MpqFraction::new();
        unsafe {
            gmp::mpq_div(&mut rop.data, &lhs.data, &rhs.data);
            gmp::mpq_canonicalize(&mut rop.data);
        }
        rop
    }

    fn internal_div_assign(&mut self, rhs: &MpqFraction) {
        unsafe {
            gmp::mpq_div(&mut self.data, &self.data, &rhs.data);
            gmp::mpq_canonicalize(&mut self.data);
        }
    }
    // endregion

    pub(crate) fn sign(&self) -> c_int {
        unsafe { gmp::mpq_sgn(&self.data) }
    }

    fn internal_hash<H: Hasher>(&self, state: &mut H) {
        // TODO implement better hash method
        self.to_string().hash(state);
    }
}

impl Drop for MpqFraction {
    fn drop(&mut self) {
        self.clear();
    }
}

impl Clone for MpqFraction {
    fn clone(&self) -> Self {
        MpqFraction::from_q(self)
    }
}

impl Display for MpqFraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl FromStr for MpqFraction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MpqFraction::from_str(s.to_string()))
    }
}

impl Zero for MpqFraction {
    fn zero() -> Self {
        MpqFraction::new()
    }

    fn is_zero(&self) -> bool {
        self.sign() == 0
    }
}

impl One for MpqFraction {
    fn one() -> Self {
        MpqFraction::from_u64(1, 1)
    }
}

impl PartialEq for MpqFraction {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(&other) == 0
    }

    fn ne(&self, other: &Self) -> bool {
        self.cmp(&other) != 0
    }
}

impl Eq for MpqFraction {}

impl Hash for MpqFraction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.internal_hash(state);
    }
}

impl Neg for MpqFraction {
    type Output = MpqFraction;

    fn neg(self) -> Self::Output {
        let mut res = MpqFraction::new();
        unsafe {
            gmp::mpq_neg(&mut res.data, &self.data);
        }
        res
    }
}

impl PartialOrd for MpqFraction {
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

// region MpzNumber
impl From<MpzNumber> for MpqFraction {
    fn from(value: MpzNumber) -> MpqFraction {
        MpqFraction::from_z(&value)
    }
}

impl From<&MpzNumber> for MpqFraction {
    fn from(value: &MpzNumber) -> MpqFraction {
        MpqFraction::from_z(&value)
    }
}

impl Add<&MpqFraction> for MpzNumber {
    type Output = MpqFraction;

    fn add(self, rhs: &MpqFraction) -> MpqFraction {
        MpqFraction::internal_add(&rhs, &MpqFraction::from_z(&self))
    }
}

impl Add<MpqFraction> for MpzNumber {
    type Output = MpqFraction;

    fn add(self, rhs: MpqFraction) -> MpqFraction {
        MpqFraction::internal_add(&rhs, &MpqFraction::from_z(&self))
    }
}

impl Mul<&MpqFraction> for MpzNumber {
    type Output = MpqFraction;

    fn mul(self, rhs: &MpqFraction) -> MpqFraction {
        MpqFraction::internal_mul(&rhs, &MpqFraction::from_z(&self))
    }
}

impl Mul<MpqFraction> for MpzNumber {
    type Output = MpqFraction;

    fn mul(self, rhs: MpqFraction) -> MpqFraction {
        MpqFraction::internal_mul(&rhs, &MpqFraction::from_z(&self))
    }
}

impl Sub<&MpqFraction> for MpzNumber {
    type Output = MpqFraction;

    fn sub(self, rhs: &MpqFraction) -> MpqFraction {
        MpqFraction::internal_sub(&MpqFraction::from_z(&self), &rhs)
    }
}

impl Sub<MpqFraction> for MpzNumber {
    type Output = MpqFraction;

    fn sub(self, rhs: MpqFraction) -> MpqFraction {
        MpqFraction::internal_sub(&MpqFraction::from_z(&self), &rhs)
    }
}

impl Div<&MpqFraction> for MpzNumber {
    type Output = MpqFraction;

    fn div(self, rhs: &MpqFraction) -> MpqFraction {
        MpqFraction::internal_div(&MpqFraction::from_z(&self), &rhs)
    }
}

impl Div<MpqFraction> for MpzNumber {
    type Output = MpqFraction;

    fn div(self, rhs: MpqFraction) -> MpqFraction {
        MpqFraction::internal_div(&MpqFraction::from_z(&self), &rhs)
    }
}
// endregion

// region From
macro_rules! impl_from_unsigned {
    ($($t:ty),* $(,)?) => {
        $(
            impl From<$t> for MpqFraction {
                fn from(value: $t) -> MpqFraction {
                    MpqFraction::from_u64(value as u64, 1)
                }
            }
        )*
    };
}

impl_from_unsigned!(u8, u16, u32, u64, usize);

macro_rules! impl_from_signed {
    ($($t:ty),*) => {
        $(
            impl From<$t> for MpqFraction {
                fn from(value: $t) -> MpqFraction {
                    MpqFraction::from_i64(value as i64, 1)
                }
            }
        )*
    }
}

impl_from_signed!(i8, i16, i32, i64, isize);
// endregion

// region Op for signed & unsigned
macro_rules! impl_op {
    ($($t:ty),*) => {
        $(
            impl Add<&MpqFraction> for $t {
                type Output = MpqFraction;

                fn add(self, rhs: &MpqFraction) -> MpqFraction {
                    MpqFraction::internal_add(&MpqFraction::from(self), &rhs)
                }
            }

            impl Add<MpqFraction> for $t {
                type Output = MpqFraction;

                fn add(self, rhs: MpqFraction) -> MpqFraction {
                    MpqFraction::internal_add(&MpqFraction::from(self), &rhs)
                }
            }

            impl Mul<&MpqFraction> for $t {
                type Output = MpqFraction;

                fn mul(self, rhs: &MpqFraction) -> MpqFraction {
                    MpqFraction::internal_mul(&rhs, &MpqFraction::from(self))
                }
            }

            impl Mul<MpqFraction> for $t {
                type Output = MpqFraction;

                fn mul(self, rhs: MpqFraction) -> MpqFraction {
                    MpqFraction::internal_mul(&rhs, &MpqFraction::from(self))
                }
            }

            impl Sub<&MpqFraction> for $t {
                type Output = MpqFraction;

                fn sub(self, rhs: &MpqFraction) -> MpqFraction {
                    MpqFraction::internal_sub(&MpqFraction::from(self), &rhs)
                }
            }

            impl Sub<MpqFraction> for $t {
                type Output = MpqFraction;

                fn sub(self, rhs: MpqFraction) -> MpqFraction {
                    MpqFraction::internal_sub(&MpqFraction::from(self), &rhs)
                }
            }

            impl Div<&MpqFraction> for $t {
                type Output = MpqFraction;

                fn div(self, rhs: &MpqFraction) -> MpqFraction {
                    MpqFraction::internal_div(&MpqFraction::from(self), &rhs)
                }
            }

            impl Div<MpqFraction> for $t {
                type Output = MpqFraction;

                fn div(self, rhs: MpqFraction) -> Self::Output {
                    MpqFraction::internal_div(&MpqFraction::from(self), &rhs)
                }
            }
        )*
    }
}

impl_op!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
// endregion

// region Addition
impl<T> Add<T> for MpqFraction
where
    T: Into<MpqFraction>,
{
    type Output = MpqFraction;
    fn add(self, rhs: T) -> Self::Output {
        MpqFraction::internal_add(&self, &rhs.into())
    }
}

impl<T> Add<T> for &MpqFraction
where
    T: Into<MpqFraction>,
{
    type Output = MpqFraction;
    fn add(self, rhs: T) -> Self::Output {
        MpqFraction::internal_add(&self, &rhs.into())
    }
}

impl<'a, 'b> Add<&'b MpqFraction> for &'a MpqFraction {
    type Output = MpqFraction;

    fn add(self, rhs: &'b MpqFraction) -> MpqFraction {
        MpqFraction::internal_add(&self, &rhs)
    }
}

impl AddAssign<&MpqFraction> for MpqFraction {
    fn add_assign(&mut self, rhs: &MpqFraction) {
        self.internal_add_assign(&rhs);
    }
}

impl<T> AddAssign<T> for MpqFraction
where
    T: Into<MpqFraction>,
{
    fn add_assign(&mut self, rhs: T) {
        self.internal_add_assign(&rhs.into());
    }
}
// endregion

// region Multiplication
impl<T> Mul<T> for MpqFraction
where
    T: Into<MpqFraction>,
{
    type Output = MpqFraction;
    fn mul(self, rhs: T) -> Self::Output {
        MpqFraction::internal_mul(&self, &rhs.into())
    }
}

impl<T> Mul<T> for &MpqFraction
where
    T: Into<MpqFraction>,
{
    type Output = MpqFraction;
    fn mul(self, rhs: T) -> Self::Output {
        MpqFraction::internal_mul(&self, &rhs.into())
    }
}

impl<'a, 'b> Mul<&'b MpqFraction> for &'a MpqFraction {
    type Output = MpqFraction;

    fn mul(self, rhs: &'b MpqFraction) -> MpqFraction {
        MpqFraction::internal_mul(&self, &rhs)
    }
}

impl MulAssign<&MpqFraction> for MpqFraction {
    fn mul_assign(&mut self, rhs: &MpqFraction) {
        self.internal_mul_assign(&rhs);
    }
}

impl<T> MulAssign<T> for MpqFraction
where
    T: Into<MpqFraction>,
{
    fn mul_assign(&mut self, rhs: T) {
        self.internal_mul_assign(&rhs.into());
    }
}
// endregion

// region Substraction
impl<T> Sub<T> for MpqFraction
where
    T: Into<MpqFraction>,
{
    type Output = MpqFraction;
    fn sub(self, rhs: T) -> Self::Output {
        MpqFraction::internal_sub(&self, &rhs.into())
    }
}

impl<T> Sub<T> for &MpqFraction
where
    T: Into<MpqFraction>,
{
    type Output = MpqFraction;
    fn sub(self, rhs: T) -> Self::Output {
        MpqFraction::internal_sub(&self, &rhs.into())
    }
}

impl<'a, 'b> Sub<&'b MpqFraction> for &'a MpqFraction {
    type Output = MpqFraction;

    fn sub(self, rhs: &'b MpqFraction) -> MpqFraction {
        MpqFraction::internal_sub(&self, &rhs)
    }
}

impl SubAssign<&MpqFraction> for MpqFraction {
    fn sub_assign(&mut self, rhs: &MpqFraction) {
        self.internal_sub_assign(&rhs);
    }
}

impl<T> SubAssign<T> for MpqFraction
where
    T: Into<MpqFraction>,
{
    fn sub_assign(&mut self, rhs: T) {
        self.internal_sub_assign(&rhs.into());
    }
}
// endregion

// region Division
impl<T> Div<T> for MpqFraction
where
    T: Into<MpqFraction>,
{
    type Output = MpqFraction;
    fn div(self, rhs: T) -> Self::Output {
        MpqFraction::internal_div(&self, &rhs.into())
    }
}

impl<T> Div<T> for &MpqFraction
where
    T: Into<MpqFraction>,
{
    type Output = MpqFraction;
    fn div(self, rhs: T) -> Self::Output {
        MpqFraction::internal_div(&self, &rhs.into())
    }
}

impl<'a, 'b> Div<&'b MpqFraction> for &'a MpqFraction {
    type Output = MpqFraction;

    fn div(self, rhs: &'b MpqFraction) -> MpqFraction {
        MpqFraction::internal_div(&self, &rhs)
    }
}

impl DivAssign<&MpqFraction> for MpqFraction {
    fn div_assign(&mut self, rhs: &MpqFraction) {
        self.internal_div_assign(&rhs);
    }
}

impl<T> DivAssign<T> for MpqFraction
where
    T: Into<MpqFraction>,
{
    fn div_assign(&mut self, rhs: T) {
        self.internal_div_assign(&rhs.into());
    }
}
// endregion

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let mpz: MpqFraction = MpqFraction::new();
        assert_eq!(mpz.get_f(), 0.0);
        assert_eq!(mpz.get_str(), "0");
    }

    #[test]
    fn test_init_ui() {
        let mpz: MpqFraction = MpqFraction::from_u64(165, 49);
        assert_eq!(mpz.get_f(), 3.36734693877551);
        assert_eq!(mpz.get_str(), "165/49");
    }

    #[test]
    fn test_init_z() {
        let q: MpqFraction = MpqFraction::from_z(&MpzNumber::binomial_ui(100, 50));
        assert_eq!(q.get_f(), 1.0089134454556418e29);
        assert_eq!(q.get_str(), "100891344545564193334812497256");

        let z = MpzNumber::factorial(20);
        let f = q / z;
        assert_eq!(f.get_f(), 41469547152.52921);
        assert_eq!(f.get_str(), "3370857116638995647473/81285120000")
    }

    #[test]
    fn test_init_zz() {
        let n = MpzNumber::binomial_ui(100, 50);
        let d = MpzNumber::factorial(20);
        let q: MpqFraction = MpqFraction::from_zz(&n, &d);
        assert_eq!(q.get_f(), 41469547152.52921);
        assert_eq!(q.get_str(), "3370857116638995647473/81285120000");
    }

    #[test]
    fn test_bin() {
        let bin = MpqFraction::from_u64(40, 20);
        assert_eq!(bin.get_f(), 2.0);

        let fac = MpqFraction::from_u64(20, 1);
        assert_eq!(fac.get_f(), 20.0);

        assert_eq!(fac > bin, true);
        assert_eq!(fac < bin, false);
        assert_eq!(fac == bin, false);
        assert_eq!(fac != bin, true);
        assert_eq!(fac.is_zero(), false);
        assert_eq!(fac.sign(), 1);
    }

    #[test]
    fn test_init_str() {
        let mpz: MpqFraction = MpqFraction::from_str("123456".to_string());
        assert_eq!(mpz.get_str(), "123456");
    }

    #[test]
    fn test_addition() {
        let n = MpqFraction::from_u64(12345678, 1);
        let m = MpqFraction::from_u64(987654321, 1);
        let r: MpqFraction = n + m;
        assert_eq!(r.get_f(), 999999999.0);
        assert_eq!(r.get_str(), "999999999");

        let nn = MpqFraction::from_str("12345678901234567890".to_string());
        let mm = MpqFraction::from_str("123456789098765432123456788765".to_string());
        let mut rr: MpqFraction = &nn + &mm;
        assert_eq!(rr.get_str(), "123456789111111111024691356655");

        rr += &r;
        assert_eq!(rr.get_str(), "123456789111111111025691356654");
        // assert_eq!(rr.get_u128(), 123456789111111111025691356654);

        rr = &r + 1234567890;
        assert_eq!(rr.get_str(), "2234567889");
        rr = 12345566 + &r;
        assert_eq!(rr.get_str(), "1012345565");
    }

    #[test]
    fn test_multiplication() {
        let n = MpqFraction::from_u64(12345678, 1);
        let m = MpqFraction::from_u64(987654321, 1);
        let r = n * m;
        assert_eq!(r.numerator().get_ui(), 12193262222374638);
        assert_eq!(r.get_str(), "12193262222374638");

        let nn = MpqFraction::from_str("12345678901234567890".to_string());
        let mm = MpqFraction::from_str("123456789098765432123456788765".to_string());
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
        let n = MpqFraction::from_u64(12345678, 1);
        let m = MpqFraction::from_u64(987654321, 1);
        let r = n - m;
        assert_eq!(r.get_f(), -975308643.0);
        assert_eq!(r.get_str(), "-975308643");

        let nn = MpqFraction::from_str("12345678901234567890".to_string());
        let mm = MpqFraction::from_str("123456789098765432123456788765".to_string());
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
        let n = MpqFraction::from_u64(12345678, 1);
        let m = MpqFraction::from_u64(987654321, 1);
        let r = m / n;
        assert_eq!(r.get_f(), 80.00000656100053);
        assert_eq!(r.get_str(), "109739369/1371742");

        let nn = MpqFraction::from_str("12345678901234567890".to_string());
        let mm = MpqFraction::from_str("123456789098765432123456788765".to_string());
        let mut rr = mm / nn;
        assert_eq!(
            rr.get_str(),
            "24691357819753086424691357753/2469135780246913578"
        );

        rr /= &r;
        assert_eq!(
            rr.get_str(),
            "16935086279191869139189486233407863/135480701249809480123626141"
        );

        rr = &r / 10;
        assert_eq!(rr.get_str(), "109739369/13717420");
        rr = 12345566123456i64 / &r;
        assert_eq!(rr.get_str(), "996172445018928256/6455257");
    }
}
