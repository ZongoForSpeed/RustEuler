use crate::maths::timer::ScopeTimer;
use crate::utils::mpz_nombre::MpzNombre;

pub fn problem020() -> u64 {
    let _timer = ScopeTimer::new("Problem 20 Factorial digit sum", false);
    // n! means n × (n − 1) × ... × 3 × 2 × 1
    //
    // For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
    // and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
    //
    // Find the sum of the digits in the number 100!
    MpzNombre::factorial(100).somme_chiffre(10)
}
