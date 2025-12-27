use crate::maths::digits::Digits;
use crate::register_problem;
use num_traits::Pow;

register_problem!(104, "Pandigital Fibonacci ends", problem104);

fn test_premier_chiffres(f: f64) -> bool {
    let chiffres = f.log10().floor();
    if chiffres < 9f64 {
        return false;
    }

    let x = 10f64.pow(chiffres - 8f64);
    let debut = (f / x) as u64;
    debut.pandigital(10)
}

fn test_dernier_chiffres(n: u64) -> bool {
    n.pandigital(10)
}

pub fn problem104() -> String {
    // The Fibonacci sequence is defined by the recurrence relation:
    //
    // Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
    // It turns out that F541, which contains 113 digits, is the first Fibonacci number for which the last nine digits
    // are 1-9 pandigital (contain all the digits 1 to 9, but not necessarily in order).
    // And F2749, which contains 575 digits, is the first Fibonacci number for which the first nine digits are 1-9
    // pandigital.
    //
    // Given that Fk is the first Fibonacci number for which the first nine digits AND the last nine digits are 1-9
    // pandigital, find k.
    let sqrt5 = 5f64.sqrt();
    let phi = (1f64 + sqrt5) / 2f64;

    let mask = 10u64.pow(9);
    let f_mask = mask as f64;
    let mut fd = phi / sqrt5;

    let mut fn1 = 0;
    let mut fn2 = 1;

    let mut result = 0;

    for n in 2.. {
        fd *= phi;
        if fd > f_mask * f_mask {
            fd /= f_mask;
        }

        (fn1, fn2) = (fn2, (fn1 + fn2) % mask);

        if test_premier_chiffres(fd) && test_dernier_chiffres(fn2) {
            result = n;
            break;
        }
    }
    result.to_string()
}
