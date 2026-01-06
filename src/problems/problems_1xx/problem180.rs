use crate::maths::polygonal::Polygonal;
use crate::register_problem;
use crate::utils::mpz_number::MpzNumber;
use fraction::Fraction;
use itertools::Itertools;
use std::collections::HashSet;

register_problem!(
    180,
    "Rational zeros of a function of three variables",
    problem180
);

pub fn problem180() -> String {
    // For any integer n, consider the three functions
    //
    // f1,n(x,y,z) = x^n+1 + y^n+1 − z^n+1
    // f2,n(x,y,z) = (xy + yz + zx)*(x^n-1 + y^n-1 − z^n-1)
    // f3,n(x,y,z) = xyz*(x^n-2 + y^n-2 − z^n-2)
    //
    // and their combination
    //
    // fn(x,y,z) = f1,n(x,y,z) + f2,n(x,y,z) − f3,n(x,y,z)
    //
    // We call (x,y,z) a golden triple of order k if x, y, and z are all rational numbers of the form a / b with
    // 0 < a < b ≤ k and there is (at least) one integer n, so that fn(x,y,z) = 0.
    //
    // Let s(x,y,z) = x + y + z.
    // Let t = u / v be the sum of all distinct s(x,y,z) for all golden triples (x,y,z) of order 35.
    // All the s(x,y,z) and t must be in reduced form.
    //
    // Find u + v.
    let limit: u64 = 35;
    let mut set_fractions = HashSet::new();
    for b in 2..=limit {
        for a in 1..b {
            set_fractions.insert(Fraction::new(a, b));
        }
    }

    let fractions: Vec<_> = set_fractions.iter().sorted().collect();

    let mut solutions = HashSet::new();
    for i in 0..fractions.len() {
        for j in i..fractions.len() {
            let x = fractions[i];
            let y = fractions[j];
            // n = 1: z = x + y
            let z = x + y;
            if set_fractions.contains(&z) {
                solutions.insert(x + y + z);
            }

            // n = -1: 1/z = 1/x + 1/y
            let z = x * y / (x + y);
            if set_fractions.contains(&z) {
                solutions.insert(x + y + z);
            }

            {
                let zz = x * x + y * y;
                if let Some(sn) = zz.numer().and_then(|z| z.get_sqrt())
                    && let Some(sd) = zz.denom().and_then(|z| z.get_sqrt())
                {
                    let z = Fraction::new(sn, sd);
                    // n = 2: z = sqrt(x^2+y^2)
                    if set_fractions.contains(&z) {
                        solutions.insert(x + y + z);
                    }
                    // n = -2: z=x*y/sqrt(x^2+y^2)
                    let z = x * y / z;
                    if set_fractions.contains(&z) {
                        solutions.insert(x + y + z);
                    }
                }
            }
        }
    }

    let mut u = MpzNumber::from(0);
    let mut v = MpzNumber::from(1);

    for s in solutions {
        if let Some(&d) = s.denom()
            && let Some(&n) = s.numer()
        {
            u = d * &u + n * &v;
            v *= d;
        }
    }

    let result = (&u + &v) / MpzNumber::gcd(&u, &v);
    result.to_string()
}
