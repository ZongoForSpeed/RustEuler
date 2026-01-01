use crate::maths::digits::Digits;
use crate::register_problem;
use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelIterator;

register_problem!(
    145,
    "How many reversible numbers are there below one-billion?",
    problem145
);

fn reversible(n: u64) -> bool {
    if n % 10 == 0 {
        return false;
    }
    let mut x = n + n.reverse(10);
    while x > 0 {
        if x & 1 == 0 {
            return false;
        }
        x /= 10;
    }
    true
}

pub fn problem145() -> String {
    // Some positive integers n have the property that the sum [ n + reverse(n) ] consists entirely of odd (decimal)
    // digits. For instance, 36 + 63 = 99 and 409 + 904 = 1313. We will call such numbers reversible; so 36, 63, 409,
    // and 904 are reversible. Leading zeroes are not allowed in either n or reverse(n).
    //
    // There are 120 reversible numbers below one-thousand.
    //
    // How many reversible numbers are there below one-billion (10^9)?
    (1..1000000000).into_par_iter().filter(|i| reversible(*i)).count().to_string()
}
