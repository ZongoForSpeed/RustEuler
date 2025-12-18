use crate::maths;
use maths::digits;
use crate::maths::timer::ScopeTimer;

pub fn problem004(borne: u64) -> u64 {
    let _timer = ScopeTimer::new("Problem 4 Largest palindrome product", false);
    // A palindromic number reads the same both ways. The largest palindrome made from the product
    // of two 2-digit numbers is 9009 = 91 × 99.
    //
    // Find the largest palindrome made from the product of two 3-digit numbers.
    let mut solution = 0;
    for a in 100..borne {
        for b in 100..a {
            let ab = a * b;
            if digits::palindrome(ab, 10) {
                solution = u64::max(solution, ab);
            }
        }
    }
    println!("solution 004: {}", solution);
    solution
}
