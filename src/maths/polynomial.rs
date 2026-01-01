use num_traits::{ConstOne, ConstZero, Signed};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use string_builder::Builder;

pub(crate) struct Polynomial<T>
where
    T: Clone
        + MulAssign
        + AddAssign
        + Copy
        + SubAssign
        + ToString
        + Signed
        + PartialOrd
        + ConstOne
        + ConstZero,
{
    _poly: Vec<T>,
}

impl<T> Polynomial<T>
where
    T: Clone
        + MulAssign
        + AddAssign
        + Copy
        + SubAssign
        + ToString
        + Signed
        + PartialOrd
        + ConstOne
        + ConstZero,
{
    pub fn new() -> Polynomial<T> {
        Polynomial { _poly: Vec::new() }
    }

    pub fn from(v: &Vec<T>) -> Polynomial<T> {
        Polynomial { _poly: v.clone() }
    }

    pub fn from_n(n: T) -> Polynomial<T> {
        Polynomial { _poly: vec![n] }
    }

    pub fn len(&self) -> usize {
        self._poly.len()
    }

    pub fn value(&self, x: T) -> T {
        let mut result = T::ZERO;
        let mut px = T::ONE;
        for p in &self._poly {
            result += px * *p;
            px *= x;
        }

        result
    }

    pub fn reduce(&mut self) {
        let mut len = self._poly.len();
        while len > 0 && self._poly[len - 1].is_zero() {
            len -= 1;
        }

        if len < self._poly.len() {
            self._poly.resize(len, T::ZERO);
        }
    }

    fn _internal_add(&mut self, p: &Polynomial<T>) {
        if p._poly.len() > self._poly.len() {
            self._poly.resize(p._poly.len(), T::ZERO);
        }
        for i in 0..p._poly.len() {
            self._poly[i] += p._poly[i];
        }
        self.reduce();
    }

    fn _internal_sub(&mut self, p: &Polynomial<T>) {
        if p._poly.len() > self._poly.len() {
            self._poly.resize(p._poly.len(), T::ZERO);
        }
        for i in 0..p._poly.len() {
            self._poly[i] -= p._poly[i];
        }
        self.reduce();
    }

    fn _internal_mul(&mut self, p: &Polynomial<T>) {
        if p._poly.is_empty() || self._poly.is_empty() {
            self._poly.clear();
            return;
        }

        let mut poly = vec![T::ZERO; self._poly.len() + p._poly.len()];
        for (i, pi) in self._poly.iter().enumerate() {
            for (j, pj) in p._poly.iter().enumerate() {
                poly[i + j] += *pi * *pj;
            }
        }
        self._poly = poly;
        self.reduce();
    }

    fn to_string(&self) -> String {
        let mut result = Builder::default();
        for (n, c) in self._poly.iter().enumerate().rev() {
            if !c.is_zero() {
                let sign = *c > T::ZERO;
                let first = result.len() == 0;
                if !first {
                    result.append(if sign { " + " } else { " - " });
                } else if !sign {
                    result.append("-");
                }
                if (c.ne(&T::ONE)) || n == 0 {
                    result.append(c.abs().to_string());
                }
                match n {
                    0 => {}
                    1 => result.append("X"),
                    _ => {
                        result.append("X^");
                        result.append(n.to_string());
                    }
                }
            }
        }
        result.string().unwrap()
    }
}

impl<T> PartialEq<Self> for Polynomial<T>
where
    T: AddAssign + Clone + Copy + MulAssign + SubAssign + ToString + Signed + PartialOrd + ConstOne + ConstZero,
{
    fn eq(&self, other: &Self) -> bool {
        self._poly == other._poly
    }
}

impl<T> PartialEq<Polynomial<T>> for &Polynomial<T>
where
    T: AddAssign + Clone + Copy + MulAssign + SubAssign + ToString + Signed + PartialOrd + ConstOne + ConstZero,
{
    fn eq(&self, other: &Polynomial<T>) -> bool {
        self._poly == other._poly
    }
}

impl<T> Eq for Polynomial<T> where
    T: Clone + MulAssign + AddAssign + Copy + SubAssign + ToString + Signed + PartialOrd + ConstOne + ConstZero
{
}

impl<T> Clone for Polynomial<T>
where
    T: Clone + MulAssign + AddAssign + Copy + SubAssign + ToString + Signed + PartialOrd + ConstOne + ConstZero,
{
    fn clone(&self) -> Self {
        Polynomial::from(&self._poly)
    }
}

impl<T> Debug for Polynomial<T>
where
    T: Clone + MulAssign + AddAssign + Copy + SubAssign + ToString + Signed + PartialOrd + ConstOne + ConstZero,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl<T> Display for Polynomial<T>
where
    T: Clone + MulAssign + AddAssign + Copy + SubAssign + ToString + Signed + PartialOrd + ConstOne + ConstZero,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl<T> AddAssign<Polynomial<T>> for Polynomial<T>
where
    T: Clone + MulAssign + AddAssign + Copy + SubAssign + ToString + Signed + PartialOrd + ConstOne + ConstZero,
{
    fn add_assign(&mut self, rhs: Polynomial<T>) {
        self._internal_add(&rhs);
    }
}

impl<T> AddAssign<&Polynomial<T>> for Polynomial<T>
where
    T: Clone + MulAssign + AddAssign + Copy + SubAssign + ToString + Signed + PartialOrd + ConstOne + ConstZero,
{
    fn add_assign(&mut self, rhs: &Polynomial<T>) {
        self._internal_add(rhs);
    }
}

impl<T> Add<&Polynomial<T>> for Polynomial<T>
where
    T: Clone + MulAssign + AddAssign + Copy + SubAssign + ToString + Signed + PartialOrd + ConstOne + ConstZero,
{
    type Output = Polynomial<T>;

    fn add(self, rhs: &Polynomial<T>) -> Polynomial<T> {
        let mut result = self.clone();
        result._internal_add(&rhs);
        result
    }
}

impl<T> Add<&Polynomial<T>> for &Polynomial<T>
where
    T: Clone + MulAssign + AddAssign + Copy + SubAssign + ToString + Signed + PartialOrd + ConstOne + ConstZero,
{
    type Output = Polynomial<T>;

    fn add(self, rhs: &Polynomial<T>) -> Polynomial<T> {
        let mut result = self.clone();
        result._internal_add(&rhs);
        result
    }
}

impl<T> MulAssign<Polynomial<T>> for Polynomial<T>
where
    T: Clone + MulAssign + AddAssign + Copy + SubAssign + ToString + Signed + PartialOrd + ConstOne + ConstZero,
{
    fn mul_assign(&mut self, rhs: Polynomial<T>) {
        self._internal_mul(&rhs);
    }
}

impl<T> MulAssign<&Polynomial<T>> for Polynomial<T>
where
    T: Clone + MulAssign + AddAssign + Copy + SubAssign + ToString + Signed + PartialOrd + ConstOne + ConstZero,
{
    fn mul_assign(&mut self, rhs: &Polynomial<T>) {
        self._internal_mul(&rhs);
    }
}

impl<T> Mul<&Polynomial<T>> for Polynomial<T>
where
    T: Clone + MulAssign + AddAssign + Copy + SubAssign + ToString + Signed + PartialOrd + ConstOne + ConstZero,
{
    type Output = Polynomial<T>;

    fn mul(self, rhs: &Polynomial<T>) -> Polynomial<T> {
        let mut result = self.clone();
        result._internal_mul(&rhs);
        result
    }
}

impl<T> Mul<&Polynomial<T>> for &Polynomial<T>
where
    T: Clone + MulAssign + AddAssign + Copy + SubAssign + ToString + Signed + PartialOrd + ConstOne + ConstZero,
{
    type Output = Polynomial<T>;

    fn mul(self, rhs: &Polynomial<T>) -> Polynomial<T> {
        let mut result = self.clone();
        result._internal_mul(&rhs);
        result
    }
}

impl<T> SubAssign<Polynomial<T>> for Polynomial<T>
where
    T: Clone + MulAssign + AddAssign + Copy + SubAssign + ToString + Signed + PartialOrd + ConstOne + ConstZero,
{
    fn sub_assign(&mut self, rhs: Polynomial<T>) {
        self._internal_sub(&rhs);
    }
}

impl<T> SubAssign<&Polynomial<T>> for Polynomial<T>
where
    T: Clone + MulAssign + AddAssign + Copy + SubAssign + ToString + Signed + PartialOrd + ConstOne + ConstZero,
{
    fn sub_assign(&mut self, rhs: &Polynomial<T>) {
        self._internal_sub(&rhs);
    }
}

impl<T> Sub<&Polynomial<T>> for Polynomial<T>
where
    T: Clone + MulAssign + AddAssign + Copy + SubAssign + ToString + Signed + PartialOrd + ConstOne + ConstZero,
{
    type Output = Polynomial<T>;

    fn sub(self, rhs: &Polynomial<T>) -> Polynomial<T> {
        let mut result = self.clone();
        result._internal_sub(&rhs);
        result
    }
}

impl<T> Sub<&Polynomial<T>> for &Polynomial<T>
where
    T: Clone + MulAssign + AddAssign + Copy + SubAssign + ToString + Signed + PartialOrd + ConstOne + ConstZero,
{
    type Output = Polynomial<T>;

    fn sub(self, rhs: &Polynomial<T>) -> Polynomial<T> {
        let mut result = self.clone();
        result._internal_sub(&rhs);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fraction::Fraction;

    #[test]
    fn test_init() {
        let p: Polynomial<i32> = Polynomial::new();
        assert_eq!(p.len(), 0);

        let p: Polynomial<i32> = Polynomial::from(&vec![10]);
        assert_eq!(p.len(), 1);
        assert_eq!(p.value(42), 10);

        let p: Polynomial<i32> = Polynomial::from(&vec![-1, 0, 1]);
        assert_eq!(p.len(), 3);
        assert_eq!(p.value(2), 3);
        assert_eq!(p.value(1), 0);
    }

    #[test]
    fn test_eq() {
        let p: Polynomial<i32> = Polynomial::from(&vec![-1, 0, 1]);
        let q: Polynomial<i32> = Polynomial::from(&vec![-1, 0, 1]);
        let r: Polynomial<i32> = Polynomial::from(&vec![1, 0, 1]);
        assert_eq!(p, q);
        assert_ne!(p, r);
    }

    #[test]
    fn test_add() {
        let p: Polynomial<i32> = Polynomial::from(&vec![-1, 0, 1]);
        let q: Polynomial<i32> = Polynomial::from(&vec![0, 1]);
        let r: Polynomial<i32> = Polynomial::from(&vec![-1, 1, 1]);
        assert_eq!(&p + &q, r);
        assert_eq!(r.to_string(), "X^2 + X - 1");
    }

    #[test]
    fn test_sub() {
        let p: Polynomial<i32> = Polynomial::from(&vec![-1, 1, 1]);
        let q: Polynomial<i32> = Polynomial::from(&vec![0, 0, 1]);
        let r: Polynomial<i32> = Polynomial::from(&vec![-1, 1]);
        assert_eq!(&p - &q, r);
        assert_eq!(r.to_string(), "X - 1");
    }

    #[test]
    fn test_mul() {
        let p: Polynomial<i32> = Polynomial::from(&vec![1, -1, 1, -1, 1]);
        let q: Polynomial<i32> = Polynomial::from(&vec![1, 1]);
        let r: Polynomial<i32> = Polynomial::from(&vec![1, 0, 0, 0, 0, 1]);
        assert_eq!(&p * &q, r);
        assert_eq!(r.to_string(), "X^5 + 1");
    }

    #[test]
    fn test_to_string() {
        let p: Polynomial<i32> = Polynomial::from(&vec![-1, 0, 1]);
        let q: Polynomial<i32> = Polynomial::from(&vec![0, 1]);
        let r: Polynomial<i32> = Polynomial::from(&vec![-1, 1, 1]);
        assert_eq!(p.to_string(), "X^2 - 1");
        assert_eq!(q.to_string(), "X");
        assert_eq!(r.to_string(), "X^2 + X - 1");
    }

    #[test]
    fn test_fraction() {
        let p: Polynomial<Fraction> = Polynomial::from(&vec![
            Fraction::from(-1),
            Fraction::from(0),
            Fraction::from(1),
        ]);
        let q: Polynomial<Fraction> = Polynomial::from(&vec![Fraction::from(0), Fraction::from(1)]);
        let r1: Polynomial<Fraction> = Polynomial::from(&vec![
            Fraction::from(0),
            Fraction::from(-1),
            Fraction::from(0),
            Fraction::from(1),
        ]);
        let r2: Polynomial<Fraction> = Polynomial::from(&vec![
            Fraction::from(-1),
            Fraction::from(-1),
            Fraction::from(1),
        ]);
        let r3: Polynomial<Fraction> = Polynomial::from(&vec![
            Fraction::from(-1),
            Fraction::from(1),
            Fraction::from(1),
        ]);

        assert_eq!(&p * &q, r1);
        assert_eq!(&p - &q, r2);
        assert_eq!(&p + &q, r3);
    }
}
