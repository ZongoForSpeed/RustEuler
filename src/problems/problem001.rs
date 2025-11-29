use crate::maths;
use maths::timer;
use timer::ScopeTimer;

pub fn problem001(borne: u32) -> u32 {
    let _timer = ScopeTimer::new("Problem 1 Multiples of 3 or 5", true);
    // If we list all the natural numbers below 10 that are multiples of 3 or 5,
    // we get 3, 5, 6 and 9. The sum of these multiples is 23.
    //
    // Find the sum of all the multiples of 3 or 5 below 1000.
    let mut solution: u32 = 0;
    for n in 1..borne {
        if n % 3 == 0 || n % 5 == 0 {
            solution += n;
        }
    }
    println!("Solution 001: {}", solution);
    solution
}
