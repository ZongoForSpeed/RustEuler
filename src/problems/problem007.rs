use crate::maths::premiers;
use crate::maths::timer::ScopeTimer;

pub fn problem007(borne: usize) -> u64 {
    let _timer = ScopeTimer::new("Problem 7 10001st prime", false);
    // By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
    //
    // What is the 10 001st prime number?
    let mut premiers: Vec<u64> = Vec::new();
    premiers::crible2(200000, |p| {
        premiers.push(p);
    });
    let solution = premiers[borne];
    println!("solution 007: {}", solution);
    solution
}
