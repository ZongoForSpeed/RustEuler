use crate::maths::timer::ScopeTimer;
use crate::utils::mpz_nombre::MpzNombre;

pub fn problem053() -> u64 {
    let _timer = ScopeTimer::new("Problem 53 Combinatoric selections", false);
    // There are exactly ten ways of selecting three from five, 12345:
    //
    // 123, 124, 125, 134, 135, 145, 234, 235, 245, and 345
    //
    // In combinatorics, we use the notation, 5C3 = 10.
    //
    // In general,
    //
    // nCr = n! / [r!(n−r)!] , where r ≤ n, n! = n×(n−1)×...×3×2×1, and 0! = 1.
    // It is not until n = 23, that a value exceeds one-million: 23C10 = 1144066.
    //
    // How many, not necessarily distinct, values of  nCr, for 1 ≤ n ≤ 100, are greater than one-million?
    let mut count = 0;
    let limit = MpzNombre::from_u64(1000000);
    for n in 1..=100 {
        for p in 0..=n {
            if MpzNombre::binomial_ui(n, p) > limit {
                count += 1;
            }
        }
    }

    count
}
