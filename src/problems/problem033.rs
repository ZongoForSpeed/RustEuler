use crate::maths::timer::ScopeTimer;
use fraction::Fraction;

pub fn problem033() -> u64 {
    let _timer = ScopeTimer::new("Problem 33 Digit cancelling fractions", false);
    // The fraction 49/98 is a curious fraction, as an inexperienced mathematician in attempting to simplify it may
    // incorrectly believe that 49/98 = 4/8, which is correct, is obtained by cancelling the 9s.
    //
    // We shall consider fractions like, 30/50 = 3/5, to be trivial examples.
    //
    // There are exactly four non-trivial examples of this type of fraction, less than one in value, and containing two
    // digits in the numerator and denominator.
    //
    // If the product of these four fractions is given in its lowest common terms, find the value of the denominator.
    let mut result: Fraction = Fraction::new(1u64, 1u64);
    for a in 1..10 {
        for b in 1..10 {
            for c in 1..10 {
                let bc:u64 = b * 10 + c;
                let ab:u64 = a * 10 + b;
                if a != b && ab * c == bc * a {
                    println!("({} / {}", ab, bc);
                    result *= Fraction::new(ab, bc);
                }
            }
        }
    }
    result.denom().unwrap().clone()
}
