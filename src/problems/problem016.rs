use crate::maths::timer::ScopeTimer;
use crate::utils::mpz_nombre::MpzNombre;

pub fn problem016() -> u64 {
    let _timer = ScopeTimer::new("Problem 16 Power digit sum", false);
    // 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
    //
    // What is the sum of the digits of the number 2^1000?
    let result =MpzNombre::puissance_ui(2, 1000).somme_chiffre(10);
    result
}
