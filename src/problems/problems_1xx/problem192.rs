use crate::maths::polygonal::Polygonal;
use crate::register_problem;
use crate::utils::mpq_fraction::MpqFraction;
use crate::utils::mpz_number::MpzNumber;
use num_traits::One;

register_problem!(192, "Best Approximations", problem192);

type Pair = (MpzNumber, MpzNumber);

fn evaluate(a: &MpzNumber, b: &MpzNumber, s: u64) -> bool {
    a * a < b * b * s
}

fn continuous_fraction(s: u64, bound: &MpzNumber) -> MpqFraction {
    let sq = s.isqrt();
    let mut a:Pair = (sq.into(), MpzNumber::one());
    let mut b:Pair = (&a.0 + 1, MpzNumber::one());

    while &a.1 + &b.1 <= *bound {
        let x = (&a.0 + &b.0, &a.1 + &b.1);
        if evaluate(&x.0, &x.1, s) {
            a = x;
        } else {
            b = x;
        }
    }

    if evaluate(&(&a.0 * &b.1 + &b.0 * &a.1), &(2 * &a.1 * &b.1), s) {
        MpqFraction::from_zz(&b.0, &b.1)
    } else {
        MpqFraction::from_zz(&a.0, &a.1)
    }
}

pub fn problem192() -> String {
    // Let x be a real number.
    // A best approximation to x for the denominator bound d is a rational number r/s in reduced
    // form, with s ≤ d, such that any rational number which is closer to x than r/s has a
    // denominator larger than d:
    //
    //                                |p/q-x| < |r/s-x| ⇒ q > d
    //
    // For example, the best approximation to √13 for the denominator bound 20 is 18/5 and the best
    // approximation to √13 for the denominator bound 30 is 101/28.
    //
    // Find the sum of all denominators of the best approximations to √n for the denominator bound
    // 10^12, where n is not a perfect square and 1 < n ≤ 100000.
    let bound = MpzNumber::from_u64(1000000000000) ;
    let mut result = MpzNumber::new();

    for n in 1..=100000 {
        if !u64::is_square(n) {
            let f = continuous_fraction(n, &bound);
            result += f.denominator();
        }
    }

    result.to_string()
}
