use crate::maths::primes;
use crate::register_problem;

register_problem!(7, "10001st prime", problem007);

pub fn problem007() -> String {
    // By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
    //
    // What is the 10 001st prime number?
    let mut primes: Vec<u64> = Vec::new();
    primes::crible2(200000, |p| {
        primes.push(p);
    });
    primes[10000].to_string()
}
