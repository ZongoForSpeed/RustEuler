use crate::maths::chiffres::boucle_chiffre;
use crate::maths::combinatoire;
use crate::maths::timer::ScopeTimer;

pub fn problem034() -> u64 {
    let _timer = ScopeTimer::new("Problem 34 Digit factorials", false);
    // 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
    //
    // Find the sum of all numbers which are equal to the sum of the factorial of their digits.
    //
    // Note: as 1! = 1 and 2! = 2 are not sums they are not included.
    let borne: u64 = 2540160; // 7*9!
    let mut solution: u64 = 0;
    for n in 10..borne {
        let mut s = 0;
        boucle_chiffre::<u64, _>(n, 10, |d| s += combinatoire::factorial(d));
        if s == n {
            solution += n;
        }
    }
    solution
}
