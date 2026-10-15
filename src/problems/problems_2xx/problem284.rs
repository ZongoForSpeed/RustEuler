use crate::register_problem;
use crate::utils::mpz_number::MpzNumber;

register_problem!(
    284,
    "Steady Squares",
    problem284
);

/// The 3-digit number 376 in the decimal numbering system is an example of numbers with the special property that
/// its square ends with the same digits: 376² = 141376. Let's call a number with this property a steady square.
///
/// Steady squares can also be observed in other numbering systems. In the base 14 numbering system, the 3-digit
/// number c37 is also a steady square: c37² = aa0c37, and the sum of its digits is c+3+7=18 in the same numbering
/// system. The letters a, b, c and d are used for the 10, 11, 12 and 13 digits respectively, in a manner similar to
/// the hexadecimal numbering system.
///
/// For 1 ≤ n ≤ 9, the sum of the digits of all the n-digit steady squares in the base 14 numbering system is 2d8
/// (582 decimal). Steady squares with leading 0's are not allowed.
///
/// Find the sum of the digits of all the n-digit steady squares in the base 14 numbering system for 1 ≤ n ≤ 10000
/// (decimal) and give your answer in the base 14 system using lower case letters where necessary.
pub fn problem284() -> String {
    const N: usize = 10000;
    let mut a: i64 = 7;
    let mut b: i64 = 8;

    let mut answer: i64 = a + b + 1;
    let mut x = MpzNumber::from_u64(7);
    let mut r = MpzNumber::from_u64(14);

    for _ in 1..N {
        let q = r.clone();
        r *= 14u64;

        let mut d: i64 = 0;
        while d < 14 {
            let carre = &x * &x;
            if carre % &r == x {
                break;
            }
            x += &q;
            d += 1;
        }
        a += d;
        b += 13 - d;
        if d != 0 {
            answer += a;
        }

        if 13 - d != 0 {
            answer += b;
        }
    }

    let mut chiffres = Vec::new();
    let mut v = answer;
    while v > 0 {
        chiffres.push(v % 14);
        v /= 14;
    }

    chiffres
        .iter()
        .rev()
        .map(|&c| match c {
            10 => 'a',
            11 => 'b',
            12 => 'c',
            13 => 'd',
            _ => (b'0' + c as u8) as char,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem284() {
        assert_eq!(problem284(), "5a411d7b");
    }
}
