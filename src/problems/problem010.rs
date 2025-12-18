use crate::maths::primes;
use crate::maths::timer::ScopeTimer;

pub fn problem010(limite: usize) -> u64 {
    let _timer = ScopeTimer::new("Problem 10 Summation of primes", false);
    // The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
    //
    // Find the sum of all the primes below two million.
    let mut solution: u64 = 0;
    primes::crible2(limite, |p: u64| solution += p);
    println!("Solution 010: {}", solution);
    solution
}
