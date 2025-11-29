use crate::maths::timer::ScopeTimer;

pub fn problem003(borne: u64) -> u64 {
    let _timer = ScopeTimer::new("Problem 3 Largest prime factor", true);
    // The prime factors of 13195 are 5, 7, 13 and 29.
    //
    // What is the largest prime factor of the number 600851475143 ?
    let mut n = borne; // 600851475143;
    let mut count = 2;
    while n != 1 {
        if n % count == 0 {
            n /= count;
        } else {
            count += 1;
        }
    }
    println!("Solution 003: {}", count);
    count
}
