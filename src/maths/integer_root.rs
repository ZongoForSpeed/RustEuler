use num_integer::Roots;
use num_traits::PrimInt;

pub fn isqrt<N: PrimInt + Roots>(y: N) -> N {
    /*
    // Integer square root (binary search)
    let mut lower = N::zero(); // lower bound of the square root
    let mut upper = y + N::one(); // upper bound of the square root
    while lower != upper - N::one() {
        let mid = (lower + upper) >> 1; // midpoint to test
        if (mid * mid <= y) {
            lower = mid;
        } else {
            upper = mid;
        }
    }
    lower
    */
    let sq = y.sqrt();
    sq
}

/*
pub fn icqrt2(n: u64) -> u64 {
    let sq = n.cbrt();
    sq * sq * sq == n
}

pub fn icbrt<N: PrimInt>(n: N) -> bool {
    let sq = n.cbrt();
    sq * sq * sq == n
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isqrt() {
        assert_eq!(isqrt(10), 3);
        assert_eq!(isqrt(9), 3);
        assert_eq!(isqrt(15241383936u64), 123456);
        assert_eq!(isqrt(123456789), 11111);
        assert_eq!(isqrt(1234567890987654321u64), 1111111106);
        assert_eq!(isqrt(15241578750190521u64), 123456789);
    }
}
