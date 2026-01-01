use num_integer::Roots;
use num_traits::PrimInt;

pub trait Polygonal: PrimInt + Roots {
    fn is_square(self) -> bool;

    fn is_triangular(self) -> bool;

    fn is_pentagonal(self) -> bool;

    fn is_hexagonal(self) -> bool;

    fn is_heptagonal(self) -> bool;

    fn is_octagonal(self) -> bool;

    fn is_polygonal(self, itype: u8) -> bool;

    fn triangular(self) -> Self;

    fn pentagonal(self) -> Self;

    fn hexagonal(self) -> Self;

    fn heptagonal(self) -> Self;

    fn octagonal(self) -> Self;

    fn polygonal(self, itype: u8) -> Self;
}

macro_rules! impl_polygonal {
    ($($t:ty),*) => {
        $(
            impl Polygonal for $t {
                fn is_square(self) -> bool {
                    let sq = self.sqrt();
                    sq * sq == self
                }

                fn is_triangular(self) -> bool {
                    let delta = 8 * self + 1;
                    let sq = delta.sqrt();
                    if sq * sq != delta {
                        return false;
                    }
                    (sq - 1) % 2 == 0
                }

                fn is_pentagonal(self) -> bool {
                    let delta = 24 * self + 1;
                    let sq = delta.sqrt();
                    if sq * sq != delta {
                        return false;
                    }
                    (sq + 1) % 6 == 0
                }

                fn is_hexagonal(self) -> bool {
                    let delta = 8 * self + 1;
                    let sq = delta.sqrt();
                    if sq * sq != delta {
                        return false;
                    }
                    (sq + 1) % 4 == 0
                }

                fn is_heptagonal(self) -> bool {
                    let delta = 40 * self  + 9;
                    let sq = delta.sqrt();
                    if sq * sq != delta {
                        return false;
                    }
                    (3 + sq) % 10 == 0
                }

                fn is_octagonal(self) -> bool {
                    let delta = 12 * self + 4;
                    let sq = delta.sqrt();
                    if sq * sq != delta {
                        return false;
                    }
                    (sq + 2) % 6 == 0
                }

                fn is_polygonal(self, itype: u8) -> bool {
                    match itype {
                        3 => self.is_triangular(),
                        4 => self.is_square(),
                        5 => self.is_pentagonal(),
                        6 => self.is_hexagonal(),
                        7 => self.is_heptagonal(),
                        8 => self.is_octagonal(),
                        _ => unreachable!(),
                    }
                }

                fn triangular(self) -> Self {
                    self * (self + 1) >> 1
                }

                fn pentagonal(self) -> Self {
                    (self * (self + self + self - 1)) >> 1
                }

                fn hexagonal(self) -> Self {
                    self * (self + self - 1)
                }

                fn heptagonal(self) -> Self {
                    (self * (5 * self - 3)) >> 1
                }

                fn octagonal(self) -> Self {
                    self * (self * 3 - 2)
                }

                fn polygonal(self, itype: u8) -> Self {
                    match itype {
                        3 => self.triangular(),
                        4 => self * self,
                        5 => self.pentagonal(),
                        6 => self.hexagonal(),
                        7 => self.heptagonal(),
                        8 => self.octagonal(),
                        _ => unreachable!(),
                    }
                }

            }
        )*
    }
}

impl_polygonal!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangulaire() {
        assert_eq!(5050, 100.triangular());
        assert!(5050.is_triangular());
        assert!(!5000.is_triangular());
    }

    #[test]
    fn test_pentagonal() {
        assert_eq!(14950, 100.pentagonal());
        assert!(3151.is_pentagonal());
        assert!(!3150.is_pentagonal());
    }

    #[test]
    fn test_hexagonal() {
        assert_eq!(19900, 100.hexagonal());
        assert!(4560.is_hexagonal());
        assert!(!4550.is_hexagonal());
    }

    #[test]
    fn test_heptagonal() {
        assert_eq!(24850, 100.heptagonal());
        assert!(5688.is_heptagonal());
        assert!(!5689.is_heptagonal());
    }

    #[test]
    fn test_octagonal() {
        assert_eq!(100.octagonal(), 29800);
        assert!(5461.is_octagonal());
        assert!(!5460.is_octagonal());
    }
}
