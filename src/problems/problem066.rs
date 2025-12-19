use crate::maths::polygonal;
use crate::maths::timer::ScopeTimer;
use crate::utils::mpz_number::MpzNumber;
use num_integer::Roots;

fn continous_fraction(d: usize) -> Vec<usize> {
    let mut result = Vec::new();
    let d_sqrt = d.sqrt();
    let mut a = d_sqrt;
    let mut p = 0;
    let mut q = 1;
    loop {
        result.push(a);
        p = a * q - p;
        q = (d - p * p) / q;
        a = (p + d_sqrt) / q;
        if q == 1 {
            break;
        }
    }
    result.push(a);
    result
}

fn pell(d: usize) -> MpzNumber {
    let c = continous_fraction(d);
    let l = c.len() - 1;
    let per = if l % 2 == 0 { l - 1 } else { 2 * l - 1 };
    let mut a: MpzNumber = c[0].into();
    let mut a1: MpzNumber = 1.into();
    let mut b: MpzNumber = a.clone();
    let mut b1: MpzNumber = 0.into();
    for i in 1..=per {
        let mut t: MpzNumber = a.clone();
        a = a * c[(i - 1) % l + 1] + a1;
        a1 = t;
        t = b.clone();
        b = b * c[(i - 1) % l + 1] + b1;
        b1 = t;
    }
    a
}

pub fn problem066() -> usize {
    let _timer = ScopeTimer::new("Problem 66 Diophantine equation", false);
    // Consider quadratic Diophantine equations of the form:
    //
    //                                      x² – Dy² = 1
    //
    // For example, when D=13, the minimal solution in x is 649² – 13×180² = 1.
    //
    // It can be assumed that there are no solutions in positive integers when D is square.
    //
    // By finding minimal solutions in x for D = {2, 3, 5, 6, 7}, we obtain the following:
    //
    //      3² – 2×2² = 1
    //      2² – 3×1² = 1
    //      9² – 5×4² = 1
    //      5² – 6×2² = 1
    //      8² – 7×3² = 1
    //
    // Hence, by considering minimal solutions in x for D ≤ 7, the largest x is obtained when D=5.
    //
    // Find the value of D ≤ 1000 in minimal solutions of x for which the largest value of x is obtained.
    let mut maximum: MpzNumber = 0.into();
    let mut result = 0;
    for d in 2..=1000 {
        if !polygonal::is_square(d) {
            let x = pell(d);
            if x > maximum
            {
                maximum = x;
                result = d;
            }
        }
    }
    result
}
