use num_integer::Roots;
use num_traits::PrimInt;

pub fn is_square<N: PrimInt + Roots>(n: N) -> bool {
    let sq = n.sqrt();
    sq * sq == n
}

pub fn triangular<N: PrimInt>(n: N) -> N {
    n * (n + N::one()) >> 1
}

pub fn is_triangular<N: PrimInt + Roots>(n: N) -> bool {
    let two = N::from(2).unwrap();
    let eight = N::from(8).unwrap();
    let delta = n * eight + N::one();
    let sq = delta.sqrt();
    if sq * sq != delta {
        return false;
    }
    (sq - N::one()) % two == N::zero()
}

pub fn pentagonal<N: PrimInt>(n: N) -> N {
    (n * (n + n + n - N::one())) >> 1
}

pub fn is_pentagonal<N: PrimInt + Roots>(n: N) -> bool {
    let i6: N = N::from(6).unwrap();
    let i24: N = N::from(24).unwrap();
    let delta = n * i24 + N::one();
    let sq = delta.sqrt();
    if sq * sq != delta {
        return false;
    }
    (sq + N::one()) % i6 == N::zero()
}

pub fn hexagonal<N: PrimInt>(n: N) -> N {
    n * (n + n - N::one())
}

pub fn is_hexagonal<N: PrimInt + Roots>(n: N) -> bool {
    let four = N::from(4).unwrap();
    let eight = N::from(8).unwrap();
    let delta = n * eight + N::one();
    let sq = delta.sqrt();
    if sq * sq != delta {
        return false;
    }
    ((sq + N::one()) % four).is_zero()
}

pub fn heptagonal<N: PrimInt>(n: N) -> N {
    let i3 = N::from(3).unwrap();
    let i5 = N::from(5).unwrap();
    (n * (n * i5 - i3)) >> 1
}

pub fn is_heptagonal<N: PrimInt + Roots>(n: N) -> bool {
    let i3: N = N::from(3).unwrap();
    let i9: N = N::from(9).unwrap();
    let i10: N = N::from(10).unwrap();
    let i40: N = N::from(40).unwrap();
    let delta = n * i40 + i9;
    let sq = delta.sqrt();
    if sq * sq != delta {
        return false;
    }
    (i3 + sq) % i10 == N::zero()
}

pub fn octagonal<N: PrimInt>(n: N) -> N {
    let i2: N = N::from(2).unwrap();
    let i3: N = N::from(3).unwrap();
    n * (n * i3 - i2)
}

pub fn is_octagonal<N: PrimInt + Roots>(n: N) -> bool {
    let i2 = N::from(2).unwrap();
    let i4 = N::from(4).unwrap();
    let i6 = N::from(6).unwrap();
    let i12 = N::from(12).unwrap();
    let delta = n * i12 + i4;
    let sq = delta.sqrt();
    if sq * sq != delta {
        return false;
    }
    (sq + i2) % i6 == N::zero()
}

pub fn polygonal<N: PrimInt>(n: N, itype: u8) -> N {
    match itype {
        3 => triangular(n),
        4 => n * n,
        5 => pentagonal(n),
        6 => hexagonal(n),
        7 => heptagonal(n),
        8 => octagonal(n),
        _ => unreachable!(),
    }
}

pub fn is_polygonal<N: PrimInt + Roots>(n: N, itype: u8) -> bool {
    match itype {
        3 => is_triangular(n),
        4 => is_square(n),
        5 => is_pentagonal(n),
        6 => is_hexagonal(n),
        7 => is_heptagonal(n),
        8 => is_octagonal(n),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangulaire() {
        assert_eq!(5050, triangular(100));
        assert!(is_triangular(5050));
        assert!(!is_triangular(5000));
    }

    #[test]
    fn test_pentagonal() {
        assert_eq!(14950, pentagonal(100));
        assert!(is_pentagonal(3151));
        assert!(!is_pentagonal(3150));
    }

    #[test]
    fn test_hexagonal() {
        assert_eq!(19900, hexagonal(100));
        assert!(is_hexagonal(4560));
        assert!(!is_hexagonal(4550));
    }

    #[test]
    fn test_heptagonal() {
        assert_eq!(24850, heptagonal(100));
        assert!(is_heptagonal(5688));
        assert!(!is_heptagonal(5689));
    }

    #[test]
    fn test_octagonal() {
        assert_eq!(29800, octagonal(100));
        assert!(is_octagonal(5461));
        assert!(!is_octagonal(5460));
    }
}
