use crate::maths;
use maths::arithmetique;
use crate::maths::timer::ScopeTimer;

pub fn problem005(borne: u64) -> u64 {
    let _timer = ScopeTimer::new("Problem 5 Smallest multiple", true);
    // 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
    //
    // What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
    let mut solution = 2;
    for d in 3..borne + 1 {
        solution = arithmetique::ppcm(solution, d);
    }
    println!("solution 005: {}", solution);
    solution
}
